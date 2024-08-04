use std::sync::Arc;
use rbatis::RBatis;
use serde_json::Value as JsonValue;
use warp::{Filter, Reply, Rejection};
use indexmap::IndexMap;
use rbs::{Value as RbsValue, value::map::ValueMap};
use std::convert::Infallible;

use crate::{auth::validar_jwt, models::response::Response};

// Función para definir la ruta del procedimiento almacenado
pub fn ruta_sp(rb: Arc<RBatis>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("SP" / String)
        .and(warp::post())
        .and(warp::header::<String>("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&rb)))
        .and_then(ejecutar_sp)
}

async fn ejecutar_sp(sp_name: String, token: String, params: JsonValue, rb: Arc<RBatis>) -> Result<impl warp::Reply, Infallible> {
    // Validar el token JWT antes de proceder con la ejecución del procedimiento almacenado
    if let Err(e) = validar_jwt(&token) {
        return Ok(warp::reply::json(&Response {
            datos: JsonValue::Null,
            estado: "error".to_string(),
            mensaje: format!("Invalid token: {:?}", e),
        }));
    }

    let mut sql = format!("EXEC {}", sp_name);
    let mut rbs_params = vec![];

    // Construir la consulta SQL y los parámetros para el procedimiento almacenado
    match params {
        JsonValue::Object(ref map) => {
            let mut first = true;
            for (k, v) in map.iter() {
                let k_str = k.to_string();
                let param = match json_to_rbs_value(&v) {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error converting parameter {} to RbsValue: {:?}", k_str, e);
                        return Ok(warp::reply::json(&Response {
                            datos: JsonValue::Null,
                            estado: "error".to_string(),
                            mensaje: format!("Error: {:?}", e),
                        }));
                    }
                };
                if !first {
                    sql.push_str(", ");
                } else {
                    sql.push_str(" ");
                }
                sql.push_str(&format!("@{} = ?", k_str));
                rbs_params.push(param);
                first = false;
            }
        }
        _ => {
            eprintln!("Invalid parameter: {:?}", params);
            return Ok(warp::reply::json(&Response {
                datos: JsonValue::Null,
                estado: "error".to_string(),
                mensaje: "Invalid parameter.".to_string(),
            }));
        }
    }

    // Ejecutar el procedimiento almacenado utilizando RBatis
    match rb.query(&sql, rbs_params).await {
        Ok(rows) => {
            Ok(warp::reply::json(&Response {
                datos: rbs_value_to_json(&rows).unwrap_or(JsonValue::Null),
                estado: "ok".to_string(),
                mensaje: "Success".to_string(),
            }))
        }
        Err(e) => {
            eprintln!("Error executing stored procedure {}: {:?}", sp_name, e);
            Ok(warp::reply::json(&Response {
                datos: JsonValue::Null,
                estado: "error".to_string(),
                mensaje: format!("Error: {:?}", e),
            }))
        }
    }
}

fn json_to_rbs_value(value: &JsonValue) -> Result<RbsValue, String> {
    // Convertir JSON a tipos RbsValue compatibles
    match value {
        JsonValue::Null => Ok(RbsValue::Null),
        JsonValue::Bool(b) => Ok(RbsValue::Bool(*b)),
        JsonValue::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(RbsValue::I64(i))
            } else if let Some(u) = num.as_u64() {
                Ok(RbsValue::U64(u))
            } else if let Some(f) = num.as_f64() {
                Ok(RbsValue::F64(f))
            } else {
                Err("Unsupported number type.".to_string())
            }
        }
        JsonValue::String(s) => Ok(RbsValue::String(s.clone())),
        JsonValue::Array(arr) => {
            let vec = arr.iter().map(json_to_rbs_value).collect::<Result<Vec<_>, _>>()?;
            Ok(RbsValue::Array(vec))
        }
        JsonValue::Object(obj) => {
            let mut map = IndexMap::new();
            for (k, v) in obj.iter() {
                map.insert(RbsValue::String(k.clone()), json_to_rbs_value(v)?);
            }
            Ok(RbsValue::Map(ValueMap(map)))
        }
    }
}

fn rbs_value_to_json(value: &RbsValue) -> Result<JsonValue, String> {
    match value {
        RbsValue::Null => Ok(JsonValue::Null),
        RbsValue::Bool(b) => Ok(JsonValue::Bool(*b)),
        RbsValue::I64(i) => Ok(JsonValue::Number(serde_json::Number::from(*i))),
        RbsValue::U64(u) => Ok(JsonValue::Number(serde_json::Number::from(*u))),
        RbsValue::F64(f) => Ok(JsonValue::Number(serde_json::Number::from_f64(*f).ok_or("Invalid f64 number")?)),
        RbsValue::String(s) => Ok(JsonValue::String(s.clone())),
        RbsValue::Array(arr) => {
            let vec = arr.iter().map(rbs_value_to_json).collect::<Result<Vec<_>, _>>()?;
            Ok(JsonValue::Array(vec))
        }
        RbsValue::Map(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                let k_str = match k {
                    RbsValue::String(s) => s.clone(),
                    _ => return Err("Unsupported map key type.".to_string()),
                };
                obj.insert(k_str, rbs_value_to_json(v)?);
            }
            Ok(JsonValue::Object(obj))
        }
        RbsValue::I32(i) => Ok(JsonValue::Number(serde_json::Number::from(*i as i64))),
        RbsValue::U32(u) => Ok(JsonValue::Number(serde_json::Number::from(*u as i64))),
        RbsValue::F32(f) => Ok(JsonValue::Number(serde_json::Number::from_f64(*f as f64).ok_or("Invalid f32 number")?)),
        RbsValue::Binary(bin) => {
            let vec = bin.iter().map(|&b| JsonValue::Number(serde_json::Number::from(b as i64))).collect();
            Ok(JsonValue::Array(vec))
        }
        RbsValue::Ext(_, _) => Err("Unsupported RbsValue extension.".to_string())
    }
}
