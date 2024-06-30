use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::auth::{generate_jwt, validate_jwt}; // Ajusta el path según la ubicación de tus funciones y estructuras

// Estructuras para las solicitudes y respuestas JSON
#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

// Controladores para manejar las rutas de autenticación
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

// Manejador para la ruta POST /login
async fn handle_login(body: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Verifica las credenciales (esto es solo un ejemplo, ajusta según tu lógica de autenticación)
    if body.username == "testuser" && body.password == "testpassword" {
        // Genera el token JWT
        match generate_jwt(&body.username) {
            Ok(token) => Ok(warp::reply::json(&LoginResponse { token })),
            Err(_) => Err(warp::reject::custom(ApiError::TokenGenerationFailed)),
        }
    } else {
        // Credenciales inválidas
        Err(warp::reject::custom(ApiError::InvalidCredentials))
    }
}

// Manejador para la ruta GET /validate
async fn handle_validate(token: String) -> Result<impl warp::Reply, warp::Rejection> {
    // Valida el token JWT
    match validate_jwt(&token) {
        Ok(claims) => Ok(warp::reply::json(&claims)),
        Err(_) => Err(warp::reject::custom(ApiError::InvalidToken)),
    }
}

// Definición de errores personalizados para manejar los rechazos de Warp
#[derive(Debug)]
enum ApiError {
    TokenGenerationFailed,
    InvalidCredentials,
    InvalidToken,
}

impl warp::reject::Reject for ApiError {} // Implementa Reject para los errores personalizados

