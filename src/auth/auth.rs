use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, DecodingKey, EncodingKey};
use std::env;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reclamaciones {
    pub sub: String,
    pub exp: usize,
}

pub fn generar_jwt(nombre_usuario: &str) -> Result<String, String> {
    let fecha_expiracion = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("marca de tiempo válida")
        .timestamp();

    let reclamaciones = Reclamaciones {
        sub: nombre_usuario.to_owned(),
        exp: fecha_expiracion as usize,
    };

    let clave_secreta = match env::var("CLAVE_SECRETA") {
        Ok(clave) => clave,
        Err(_) => return Err("CLAVE_SECRETA debe estar configurada".to_string()),
    };

    encode(&Header::default(), &reclamaciones, &EncodingKey::from_secret(clave_secreta.as_ref()))
        .map_err(|e| format!("Error al generar JWT: {:?}", e))
}

pub fn validar_jwt(token: &str) -> Result<Reclamaciones, Box<dyn Error>> {
    let clave_secreta = env::var("CLAVE_SECRETA").map_err(|_| "CLAVE_SECRETA debe estar configurada")?;
    decode::<Reclamaciones>(&token, &DecodingKey::from_secret(clave_secreta.as_ref()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::env;

    #[test]
    fn probar_generar_y_validar_jwt() {
        // Establecer una clave secreta de ejemplo
        env::set_var("CLAVE_SECRETA", "mi_clave_secreta");

        // Generar un JWT
        let nombre_usuario = "usuario_de_prueba";
        let token = generar_jwt(nombre_usuario).expect("Error al generar JWT");

        // Validar el JWT
        let reclamaciones = validar_jwt(&token).expect("Error al validar JWT");
        assert_eq!(reclamaciones.sub, nombre_usuario);
    }
}
