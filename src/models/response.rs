use serde::Serialize;
use rbs::Value;

#[derive(Serialize)]
pub struct Response {
    pub datos: Vec<Value>,
    pub estado: String,
    pub mensaje: String,
}
