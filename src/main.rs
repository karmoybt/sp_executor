use warp::Filter;
use std::sync::Arc;

mod config;
mod routes;
mod models;
mod auth;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Inicializa la base de datos y envuélvela en un Arc
    let rb = Arc::new(config::database::init_db().await);

    // Obtén las rutas desde los módulos correspondientes
    let db_routes = routes::database::db_route(rb.clone());
    let sp_routes = routes::stored_procedure::sp_route(rb.clone());
    let auth_routes = routes::auth_routes::auth_routes();

    // Combina todas las rutas
    let routes = db_routes.or(sp_routes).or(auth_routes);

    // Obtiene la dirección del servidor desde config::server
    let server_addr = config::server::get_server_addr();

    // Inicia el servidor con las rutas definidas
    warp::serve(routes).run(server_addr).await;
}
