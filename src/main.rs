use warp::Filter;
use std::sync::Arc;

mod config;
mod routes;
mod models;
mod auth;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    //Conectar a la Base de datos
    let rb = Arc::new(config::database::inicializar_bd().await);

    //Ruta de prueba para porbar la conexion a la bbdd
   // let rutas_bd = routes::database::ruta_bd(rb.clone());
    let rutas_sp = routes::stored_procedure::ruta_sp(rb.clone()); 
    let rutas_autenticacion = routes::auth_routes::rutas_autenticacion();

    //let rutas = rutas_bd.or(rutas_sp).or(rutas_autenticacion);
    let rutas = rutas_sp.or(rutas_autenticacion);

        

    let direccion_servidor = config::server::obtener_direccion_del_servidor();

    warp::serve(rutas).run(direccion_servidor).await;
}
