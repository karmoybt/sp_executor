use rbatis::{rbatis::RBatis, DefaultPool};
use rbdc_mssql::{tiberius::Config, MssqlConnectOptions, MssqlDriver};
use std::env;

pub async fn inicializar_bd() -> RBatis {
    let rb = RBatis::new();
    let url_bd = env::var("URL_DE_LA_BASE_DE_DATOS").expect("URL_DE_LA_BASE_DE_DATOS debe estar configurada");

    println!("Conectando a la base de datos con URL: {}", url_bd);

    let configuración = Config::from_jdbc_string(&url_bd).expect("Error al analizar la URL de JDBC");
    let opciones = MssqlConnectOptions(configuración);

    match rb.init_option::<MssqlDriver, MssqlConnectOptions, DefaultPool>(MssqlDriver {}, opciones) {
        Ok(_) => println!("Conectado a la base de datos"),
        Err(e) => panic!("Error al conectar a la base de datos: {:?}", e),
    }

    rb
}

// #[cfg(test)]
// mod pruebas {
//     use super::*;

//     #[tokio::test]
//     async fn probar_inicializar_bd() {
//         let _ = env::var("URL_DE_LA_BASE_DE_DATOS").unwrap_or_else(|_| {
//             env::set_var("URL_DE_LA_BASE_DE_DATOS", "mssql://sa:root@DESKTOP-KQA9AVC:1433/MiTest");
//             "URL_DE_LA_BASE_DE_DATOS set".to_string()
//         });

//         let rb = inicializar_bd().await;
//         assert!(rb.is_initialized());
//     }
// }

