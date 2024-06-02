use warp::Filter;
use std::sync::Arc;
use rbatis::rbatis::RBatis;
use serde_json::Value as JsonValue; // Alias para el tipo de serde_json::Value
use rbs::Value as RbsValue; // Alias para el tipo de rbs::Value
use crate::models::response::Response;
use std::convert::Infallible;

pub fn sp_route(rb: Arc<RBatis>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("SP" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&rb)))
        .and_then(call_stored_procedure)
}

async fn call_stored_procedure(sp_name: String, params: JsonValue, rb: Arc<RBatis>) -> Result<impl warp::Reply, Infallible> {
    let sql = format!("EXEC {}", sp_name);

    // Convertir params de serde_json::Value a rbs::Value
    let rbs_params: RbsValue = match serde_json::from_value(params) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Failed to convert params to rbs::Value: {:?}", e);
            return Ok(warp::reply::json(&Response {
                data: RbsValue::Null.into(),
                status: "error".to_string(),
                message: format!("Error: {:?}", e),
            }));
        }
    };

    // Ejecutar la consulta y manejar los errores
    match rb.query(&sql, vec![rbs_params]).await {
        Ok(rows) => {
            Ok(warp::reply::json(&Response {
                data: rows.into(),
                status: "ok".to_string(),
                message: "Success".to_string(),
            }))
        }
        Err(e) => {
            eprintln!("Stored procedure {} failed: {:?}", sp_name, e);
            Ok(warp::reply::json(&Response {
                data: RbsValue::Null.into(),
                status: "error".to_string(),
                message: format!("Error: {:?}", e),
            }))
        }
    }
}
