use rbatis::{rbatis::RBatis, DefaultPool};
use rbdc_mssql::{tiberius::Config, MssqlConnectOptions, MssqlDriver};
use std::env;

pub async fn init_db() -> RBatis {
    let rb = RBatis::new();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to database with URL: {}", db_url);

    let config = Config::from_jdbc_string(&db_url).expect("Error parsing JDBC URL");
    let options = MssqlConnectOptions(config);

    match rb.init_option::<MssqlDriver, MssqlConnectOptions, DefaultPool>(MssqlDriver {}, options) {
        Ok(_) => println!("Connected to database"),
        Err(e) => panic!("Database connection failed: {:?}", e),
    }

    rb
}
