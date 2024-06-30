use warp::Filter;
use crate::auth::validate_jwt;
use crate::models::response::Response;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use std::convert::Infallible;
use indexmap::IndexMap;
use rbs::value::map::ValueMap;
use rbs::Value as RbsValue;
use rbatis::rbatis::RBatis;

pub fn ruta_sp(rb: Arc<RBatis>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("SP" / String)
        .and(warp::post())
        .and(warp::header::<String>("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&rb)))
        .and_then(ejecutar_sp)
}

async fn ejecutar_sp(sp_name: String, token: String, params: JsonValue, rb: Arc<RBatis>) -> Result<impl warp::Reply, Infallible> {
    // Validar el token JWT antes de proceder con la ejecución del procedimiento almacenado
    if let Err(e) = validate_jwt(&token) {
        return Ok(warp::reply::json(&Response {
            data: RbsValue::Null.into(),
            status: "error".to_string(),
            message: format!("Invalid token: {:?}", e),
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
                            data: RbsValue::Null.into(),
                            status: "error".to_string(),
                            message: format!("Error: {:?}", e),
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
                data: RbsValue::Null.into(),
                status: "error".to_string(),
                message: "Invalid parameter.".to_string(),
            }));
        }
    }

    // Ejecutar el procedimiento almacenado utilizando RBatis
    match rb.query(&sql, rbs_params).await {
        Ok(rows) => {
            Ok(warp::reply::json(&Response {
                data: rows.into(),
                status: "ok".to_string(),
                message: "Success".to_string(),
            }))
        }
        Err(e) => {
            eprintln!("Error executing stored procedure {}: {:?}", sp_name, e);
            Ok(warp::reply::json(&Response {
                data: RbsValue::Null.into(),
                status: "error".to_string(),
                message: format!("Error: {:?}", e),
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
