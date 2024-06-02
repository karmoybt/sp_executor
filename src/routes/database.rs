use warp::Filter;
use std::sync::Arc;
use rbatis::rbatis::RBatis;
use crate::models::response::Response;

pub fn db_route(rb: Arc<RBatis>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("Test" / "BBDD")
        .and(warp::get())
        .and(warp::any().map(move || Arc::clone(&rb)))
        .map(|_rb: Arc<RBatis>| {
            let database_name = std::env::var("DATABASE_URL").unwrap().to_string();
            let response = Response {
                data: Vec::new(),
                status: "ok".to_string(),
                message: format!("Connected to database: {}", database_name),
            };
            warp::reply::json(&response)
        })
}
