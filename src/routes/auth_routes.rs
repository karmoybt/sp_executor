use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::auth::{generar_jwt, validar_jwt}; 

// Estructuras para las solicitudes y respuestas JSON
#[derive(Serialize, Deserialize)]
struct SolicitudDeInicioDeSesión {
    nombre_usuario: String,
    contraseña: String,
}

#[derive(Serialize)]
struct RespuestaDeInicioDeSesión {
    token: String,
}

// Controladores para manejar las rutas de autenticación
pub fn rutas_autenticacion() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let ruta_de_inicio_de_sesión = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_comprobar);

    let ruta_de_validación = warp::path("validar")
        .and(warp::header::<String>("authorization"))
        .and_then(login_validar);

    ruta_de_inicio_de_sesión.or(ruta_de_validación)
}

// Manejador para la ruta POST /login
async fn login_comprobar(cuerpo: SolicitudDeInicioDeSesión) -> Result<impl warp::Reply, warp::Rejection> {
    // Verifica las credenciales (esto es solo un ejemplo, ajusta según tu lógica de autenticación)
    println!("hola");
    if cuerpo.nombre_usuario == "testuser" && cuerpo.contraseña == "testpassword" {
        // Genera el token JWT
        match generar_jwt(&cuerpo.nombre_usuario) {
            Ok(token) => Ok(warp::reply::json(&RespuestaDeInicioDeSesión { token })),
            Err(_) => Err(warp::reject::custom(ErrorDeApi::FallaAlGenerarToken)),
        }

    } else {
        // Credenciales inválidas
        Err(warp::reject::custom(ErrorDeApi::CredencialesInvalidas))
    }
}

// Manejador para la ruta GET /validar
async fn login_validar(token: String) -> Result<impl warp::Reply, warp::Rejection> {
    // Valida el token JWT
    match validar_jwt(&token) {
        Ok(reclamaciones) => Ok(warp::reply::json(&reclamaciones)),
        Err(_) => Err(warp::reject::custom(ErrorDeApi::TokenInvalido)),
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
