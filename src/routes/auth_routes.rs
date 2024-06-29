use warp::Filter;
use crate::auth::auth::{create_jwt, validate_jwt, CustomError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

pub fn auth_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_login);

    let validate_route = warp::path("validate")
        .and(warp::header::<String>("authorization"))
        .and_then(handle_validate);

    login_route.or(validate_route)
}

async fn handle_login(body: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    if body.username == "testuser" && body.password == "testpassword" {
        match create_jwt(&body.username) {
            Ok(token) => Ok(warp::reply::json(&token)),
            Err(_) => Err(warp::reject::custom(CustomError)),
        }
    } else {
        Err(warp::reject::custom(CustomError))
    }
}

async fn handle_validate(token: String) -> Result<impl warp::Reply, warp::Rejection> {
    match validate_jwt(&token) {
        Ok(claims) => Ok(warp::reply::json(&claims)),
        Err(_) => Err(warp::reject::custom(CustomError)),
    }
}
