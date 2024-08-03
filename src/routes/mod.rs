pub mod database;
pub mod stored_procedure;
pub mod auth_routes;



//use warp::Filter;


// pub fn routes(rb: std::sync::Arc<crate::config::database::Database>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     let stored_procedures = stored_procedure::sp_route(rb.clone());
//     let database_routes = database::db_route(rb.clone());

//     stored_procedures.or(database_routes)
// }
