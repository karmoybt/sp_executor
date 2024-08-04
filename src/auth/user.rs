// use std::sync::Arc;
// use rbatis::RBatis;
// use warp::{Reply, Rejection, http::StatusCode};
// use serde::Deserialize;
// use serde_json::json;

// use crate::auth::hash_password;

// #[derive(Deserialize)]
// struct UserCreationRequest {
//     username: String,
//     password: String,
//     email: String,
// }

// async fn create_user_handler(user: UserCreationRequest, rb: Arc<RBatis>) -> Result<impl Reply, Rejection> {
//     let salt = generate_salt();
//     let password_hash = hash_password(&user.password, &salt);

//     let sp_params = json!({
//         "Username": user.username,
//         "PasswordHash": password_hash,
//         "Email": user.email,
//         "Salt": salt,
//         "IsActive": true
//     });

//     match ejecutar_sp(rb, "CreateUser", sp_params).await {
//         Ok(_) => Ok(warp::reply::with_status("User created".to_string(), StatusCode::CREATED)),
//         Err(e) => Ok(warp::reply::with_status(
//             format!("Error: {:?}", e),
//             StatusCode::INTERNAL_SERVER_ERROR,
//         )),
//     }
// }

// fn generate_salt() -> String {
//     use rand::{distributions::Alphanumeric, Rng};
//     let salt: String = rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(16)
//         .map(char::from)
//         .collect();
//     salt
// }
