use warp::Filter;
use crate::auth::{create_jwt, validate_jwt};
use serde::{Deserialize, Serialize};
use warp::http::StatusCode;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login_handler(login: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Aquí deberías validar las credenciales del usuario (omitiendo por simplicidad)
    // Si las credenciales son válidas, crea un JWT
    let token = create_jwt(&login.username).map_err(|_| warp::reject::custom("Error creating token"))?;

    Ok(warp::reply::json(&LoginResponse { token }))
}

async fn protected_handler(token: String) -> Result<impl warp::Reply, warp::Rejection> {
    validate_jwt(&token).map_err(|_| warp::reject::custom("Invalid token"))?;
    Ok(StatusCode::OK)
}

pub fn auth_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let protected = warp::path("protected")
        .and(warp::header::<String>("authorization"))
        .and_then(protected_handler);

    login.or(protected)
}
