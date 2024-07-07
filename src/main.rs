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
    let rb = Arc::new(config::database::inicializar_bd().await);

    // Obtén las rutas desde los módulos correspondientes
    let rutas_bd =routes::database::ruta_bd(rb.clone());
    let rutas_sp =routes::stored_procedure::ruta_procedimiento_almacenado(rb.clone());
    let rutas_autenticacion =routes::auth_routes::rutas_autenticacion();

    // Combina todas las rutas
    let rutas = rutas_bd.or(rutas_sp).or(rutas_autenticacion);

    // Obtiene la dirección del servidor desde configuracion::servidor
    let direccion_servidor = config::server::obtener_direccion_servidor();

    // Inicia el servidor con las rutas definidas
    warp::serve(rutas).run(direccion_servidor).await;
}
