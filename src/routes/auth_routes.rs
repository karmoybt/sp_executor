use warp::{Filter, Rejection, Reply, reject, http::StatusCode};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
//use std::env;
use warp::http::Method;
use crate::auth::{generar_jwt, validar_jwt}; 

// Estructuras para las solicitudes y respuestas JSON
#[derive(Serialize, Deserialize)]
struct SolicitudDeInicioDeSesión {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RespuestaDeInicioDeSesión {
    token: String,
}

#[derive(Serialize)]
struct FallaSession {
    error: String,
}

pub fn rutas_autenticacion() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Cargar variables de entorno
    dotenv().ok();
    //let frontend_origin = env::var("FRONT").expect("FRONT environment variable is not set");

    let cors = warp::cors()
        //.allow_origin(frontend_origin.as_str()) 
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    let ruta_de_inicio_de_sesión = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_comprobar)
        .recover(manejar_rechazo)
        .with(cors.clone());

    let ruta_de_validación = warp::path("validar")
        .and(warp::header::<String>("authorization"))
        .and_then(login_validar)
        .recover(manejar_rechazo)
        .with(cors);

    // Combina las rutas en un solo filtro
    ruta_de_inicio_de_sesión.or(ruta_de_validación)
}

// Manejador para la ruta POST /login
async fn login_comprobar(cuerpo: SolicitudDeInicioDeSesión) -> Result<impl warp::Reply, warp::Rejection> {
    if cuerpo.username == "testuser" && cuerpo.password == "testpassword" {
        match generar_jwt(&cuerpo.username) {
            Ok(token) => Ok(warp::reply::json(&RespuestaDeInicioDeSesión { token })),
            Err(_) => Err(reject::custom(ErrorDeApi::FallaAlGenerarToken)),
        }
    } else {
        Err(reject::custom(ErrorDeApi::CredencialesInvalidas))
    }
}

// Manejador para la ruta POST /validar
async fn login_validar(token: String) -> Result<impl warp::Reply, warp::Rejection> {
    match validar_jwt(&token) {
        Ok(reclamaciones) => Ok(warp::reply::json(&reclamaciones)),
        Err(_) => Err(reject::custom(ErrorDeApi::TokenInvalido)),
    }
}

// Definición de errores personalizados para manejar los rechazos de Warp
#[derive(Debug)]
enum ErrorDeApi {
    FallaAlGenerarToken,
    CredencialesInvalidas,
    TokenInvalido,
}

impl warp::reject::Reject for ErrorDeApi {} // Implementa Reject para los errores personalizados

// Función para manejar rechazos y convertirlos en respuestas JSON
async fn manejar_rechazo(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = err.find::<ErrorDeApi>() {
        let (code, message) = match error {
            ErrorDeApi::FallaAlGenerarToken => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token"),
            ErrorDeApi::CredencialesInvalidas => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            ErrorDeApi::TokenInvalido => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };

        let json = warp::reply::json(&FallaSession {
            error: message.to_string(),
        });

        return Ok(warp::reply::with_status(json, code));
    }

    // Si no es un error que podamos manejar, simplemente reenvía el rechazo
    Err(err)
}
