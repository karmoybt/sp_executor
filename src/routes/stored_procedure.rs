use std::sync::Arc;
use rbatis::RBatis;
use serde_json::Value as JsonValue;
use warp::{Filter, Reply, Rejection, http::StatusCode};
use indexmap::IndexMap;
use rbs::{Value as RbsValue, value::map::ValueMap};

// Función para ejecutar un procedimiento almacenado
pub async fn execute_stored_procedure(rb: Arc<RBatis>, sp_name: &str, params: JsonValue) -> Result<(), String> {
    let mut sql = format!("EXEC {}", sp_name);
    let mut parametros_rbs = vec![];

    match params {
        JsonValue::Object(ref mapa) => {
            let mut first = true;
            for (k, v) in mapa.iter() {
                let k_str = k.to_string();
                let parametro = json_to_rbs_value(v)?;
                if !first {
                    sql.push_str(", ");
                } else {
                    sql.push_str(" ");
                }
                sql.push_str(&format!("@{} = ?", k_str));
                parametros_rbs.push(parametro);
                first = false;
            }
        }
        _ => return Err("Invalid parameter".to_string()),
    }

    rb.query(&sql, parametros_rbs).await.map_err(|e| e.to_string())?;

    Ok(())
}

// Función para definir la ruta del procedimiento almacenado
pub fn ruta_procedimiento_almacenado(rb: Arc<RBatis>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("execute_sp" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || rb.clone()))
        .and_then(move |sp_name: String, params: JsonValue, rb: Arc<RBatis>| async move {
            match execute_stored_procedure(rb, &sp_name, params).await {
                Ok(_) => Ok::<_, Rejection>(warp::reply::with_status("Success".to_string(), StatusCode::OK)),
                Err(e) => Ok::<_, Rejection>(warp::reply::with_status(
                    format!("Error: {:?}", e),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            }
        })
}

// Función para convertir JSON a tipos RbsValue
fn json_to_rbs_value(value: &JsonValue) -> Result<RbsValue, String> {
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
