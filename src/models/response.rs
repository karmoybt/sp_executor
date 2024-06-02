use serde::Serialize;
use rbs::Value;

#[derive(Serialize)]
pub struct Response {
    pub data: Vec<Value>,
    pub status: String,
    pub message: String,
}
