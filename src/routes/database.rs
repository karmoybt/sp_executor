// use warp::Filter;
// use std::sync::Arc;
// use rbatis::rbatis::RBatis;
// use crate::models::response::Response;

// pub fn ruta_bd(rb: Arc<RBatis>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("Prueba" / "BBDD")
//         .and(warp::get())
//         .and(warp::any().map(move || Arc::clone(&rb)))
//         .map(|_rb: Arc<RBatis>| {
//             let nombre_bd = std::env::var("URL_DE_LA_BASE_DE_DATOS").unwrap().to_string();
//             let respuesta = Response {
//                 datos: JsonValue::Null,
//                 estado: "ok".to_string(),
//                 mensaje: format!("Conectado a la base de datos: {}", nombre_bd),
//             };
//             warp::reply::json(&respuesta)
//         })
// }


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use warp::test::request;

//     #[tokio::test]
//     async fn test_ruta_bd() {
//         let rb = Arc::new(RBatis::new());
//         let filter = ruta_bd(rb);
//         let resp = request()
//             .method("GET")
//             .path("/Prueba/BBDD")
//             .reply(filter)
//             .await
//             .unwrap();
//         assert_eq!(resp.status(), 200);
//         let body = resp.body().clone();
//         let respuesta: Response = serde_json::from_slice(&body).unwrap();
//         assert_eq!(respuesta.estado, "ok");
//         assert!(respuesta.mensaje.contains("Conectado a la base de datos:"));
//     }
// }
