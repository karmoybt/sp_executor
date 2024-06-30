pub mod auth;  // Marca el módulo auth como público

pub use auth::validate_jwt;
pub use auth::generate_jwt;
