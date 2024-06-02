use warp::Filter;
use std::sync::Arc;
use rbatis::rbatis::RBatis;
use serde_json::json;
use crate::models::response::Response;
use std::convert::Infallible;
use rbs::Value;

pub fn sp_route(rb: Arc<RBatis>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("SP" / String)
        .and(warp::get())
        .and(warp::any().map(move || Arc::clone(&rb)))
        .and_then(call_stored_procedure)
}

async fn call_stored_procedure(sp_name: String, rb: Arc<RBatis>) -> Result<impl warp::Reply, Infallible> {
    let sql = format!("EXEC {}", sp_name);

    let rows: Vec<Value> = match rb.query(&sql, vec![]).await {
        Ok(rows) => rows.into(),
        Err(e) => {
            eprintln!("Stored procedure {} failed: {:?}", sp_name, e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "status": "error", "message": format!("Error: {:?}", e) })),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&Response {
            data: rows,
            status: "ok".to_string(),
            message: "Success".to_string(),
        }),
        warp::http::StatusCode::OK,
    ))
}
