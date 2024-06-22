use warp::Filter;
use std::sync::Arc;

mod config;
mod routes;
mod models;
mod auth;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let rb = Arc::new(config::database::init_db().await);

    let db_route = routes::database::db_route(rb.clone());
    let sp_route = routes::stored_procedure::sp_route(rb.clone());
    let auth_routes = routes::auth_routes::auth_routes();

    let routes = auth_routes.or(db_route).or(sp_route);

    let server_addr = config::server::get_server_addr();

    warp::serve(routes).run(server_addr).await;
}
