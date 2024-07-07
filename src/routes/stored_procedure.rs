use warp::Filter;
use crate::auth::validar_jwt;
use crate::models::response::Response;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use std::convert::Infallible;
use indexmap::IndexMap;
use rbs::value::map::ValueMap;
use rbs::Value as RbsValue;
use rbatis::rbatis::RBatis;

pub fn ruta_procedimiento_almacenado(rb: Arc<RBatis>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("SP" / String)
        .and(warp::post())
        .and(warp::header::<String>("authorization"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&rb)))
        .and_then(ejecutar_procedimiento_almacenado)
}

async fn ejecutar_procedimiento_almacenado(nombre_sp: String, token: String, params: JsonValue, rb: Arc<RBatis>) -> Result<impl warp::Reply, Infallible> {
    // Validar el token JWT antes de proceder con la ejecución del procedimiento almacenado
    if let Err(e) = validar_jwt(&token) {
        return Ok(warp::reply::json(&Response {
            datos: RbsValue::Null.into(),
            estado: "error".to_string(),
            mensaje: format!("Token no válido: {:?}", e),
        }));
    }

    let mut sql = format!("EXEC {}", nombre_sp);
    let mut parametros_rbs = vec![];

    // Construir la consulta SQL y los parámetros para el procedimiento almacenado
    match params {
        JsonValue::Object(ref mapa) => {
            let mut first = true;
            for (k, v) in mapa.iter() {
                let k_str = k.to_string();
                let parametro = match json_to_rbs_value(&v) {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error convirtiendo parámetro {} a RbsValue: {:?}", k_str, e);
                        return Ok(warp::reply::json(&Response {
                            datos: RbsValue::Null.into(),
                            estado: "error".to_string(),
                            mensaje: format!("Error: {:?}", e),
                        }));
                    }
                };
                if !first {
                    sql.push_str(", ");
                } else {
                    sql.push_str(" ");
                }
                sql.push_str(&format!("@{} = ?", k_str));
                parametros_rbs.push(parametro);
                first = false;
            }
        }
        _ => {
            eprintln!("Parámetro no válido: {:?}", params);
            return Ok(warp::reply::json(&Response {
                datos: RbsValue::Null.into(),
                estado: "error".to_string(),
                mensaje: "Parámetro no válido.".to_string(),
            }));
        }
    }

    // Ejecutar el procedimiento almacenado utilizando RBatis
    match rb.query(&sql, parametros_rbs).await {
        Ok(filas) => {
            Ok(warp::reply::json(&Response {
                datos: filas.into(),
                estado: "ok".to_string(),
                mensaje: "Éxito".to_string(),
            }))
        }
        Err(e) => {
            eprintln!("Error ejecutando procedimiento almacenado {}: {:?}", nombre_sp, e);
            Ok(warp::reply::json(&Response {
                datos: RbsValue::Null.into(),
                estado: "error".to_string(),
                mensaje: format!("Error: {:?}", e),
            }))
        }
    }
}

fn json_to_rbs_value(valor: &JsonValue) -> Result<RbsValue, String> {
    // Convertir JSON a tipos RbsValue compatibles
    match valor {
        JsonValue::Null => Ok(RbsValue::Null),
        JsonValue::Bool(b) => Ok(RbsValue::Bool(*b)),
        JsonValue::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(RbsValue::I64(i))
            } else if let Some(u) = num.as_u64() {
                Ok(RbsValue::U64(u))
            } else if let Some(f) = num.as_f64() {
                Ok(RbsValue::F64(f))
            } else {
                Err("Tipo de número no soportado.".to_string())
            }
        }
        JsonValue::String(s) => Ok(RbsValue::String(s.clone())),
        JsonValue::Array(arr) => {
            let vec = arr.iter().map(json_to_rbs_value).collect::<Result<Vec<_>, _>>()?;
            Ok(RbsValue::Array(vec))
        }
        JsonValue::Object(obj) => {
            let mut mapa = IndexMap::new();
            for (k, v) in obj.iter() {
                mapa.insert(RbsValue::String(k.clone()), json_to_rbs_value(v)?);
            }
            Ok(RbsValue::Map(ValueMap(mapa)))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use warp::test::request;

//     #[tokio::test]
//     async fn test_ruta_procedimiento_almacenado() {
//         let rb = Arc::new(RBatis::new());
//         let filter = ruta_procedimiento_almacenado(rb);
//         let resp = request()
//             .method("POST")
//             .path("/SP/mi_procedimiento_almacenado")
//             .header("authorization", "Bearer my_token")
//             .json(&serde_json::json!({
//                 "param1": "valor1",
//                 "param2": 2,
//             }))
//             .reply(filter)
//             .await
//             .unwrap();
//         assert_eq!(resp.status(), 200);
//         let body = resp.body().clone();
//         let respuesta: Response = serde_json::from_slice(&body).unwrap();
//         assert_eq!(respuesta.estado, "ok");
//         assert!(respuesta.mensaje.contains("Éxito"));
//     }
// }
