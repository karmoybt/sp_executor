pub mod auth;  // Marca el módulo auth como público

pub use auth::validar_jwt;
pub use auth::generar_jwt;
