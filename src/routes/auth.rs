use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, DecodingKey, EncodingKey};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Reclamaciones {
    sub: String,
    exp: usize,
}

pub fn generar_jwt(nombre_usuario: &str) -> Result<String, String> {
    let fecha_expiración = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("marca de tiempo válida")
        .timestamp();

    let reclamaciones = Reclamaciones {
        sub: nombre_usuario.to_owned(),
        exp: fecha_expiración as usize,
    };

    let clave_secreta = env::var("CLAVE_SECRETA").expect("CLAVE_SECRETA debe estar configurada");
    encode(&Header::default(), &reclamaciones, &EncodingKey::from_secret(clave_secreta.as_ref()))
        .map_err(|e| format!("Error al generar JWT: {:?}", e))
}

pub fn validar_jwt(token: &str) -> Result<Reclamaciones, String> {
    let clave_secreta = env::var("CLAVE_SECRETA").expect("CLAVE_SECRETA debe estar configurada");
    decode::<Reclamaciones>(&token, &DecodingKey::from_secret(clave_secreta.as_ref()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| format!("Error al validar JWT: {:?}", e))
}
