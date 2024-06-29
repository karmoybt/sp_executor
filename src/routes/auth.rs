use warp::Filter;
use crate::auth::{create_jwt, validate_jwt};
use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use jsonwebtoken::{encode, Header};
use crate::models::user::User;
use chrono::{Utc, Duration};
use std::env;


#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

// Función para generar un token JWT dado un usuario
pub fn generate_jwt(user: &User) -> Result<String, String> {
    let expiration = Utc::now().checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.username.clone(),
        exp: expiration as usize,
    };

    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    encode(&Header::default(), &claims, &secret.as_ref())
        .map_err(|e| format!("Error generando token JWT: {:?}", e))
}

// Función para manejar el proceso de login
pub fn login(user: &User) -> Result<String, String> {
    // Aquí deberías verificar las credenciales del usuario
    // En este ejemplo, simplemente generamos un token JWT
    generate_jwt(user)
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
