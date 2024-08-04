use serde::Serialize;
use serde_json::Value as JsonValue;

#[derive(Serialize)]
pub struct Response {
    pub datos: JsonValue,
    pub estado: String,
    pub mensaje: String,
}
