use rbatis::rbatis;
use std::env;

pub async fn init_rbatis() -> Result<Rbatis, _> {
    let rb = Rbatis::new();
    //let database_url = "mssql://sa:root@DESKTOP-KQA9AVC:1433/MiTest";
    let database_url = env::var("DATABASE_NAME");
    rb.link(&database_url).await.expect("Failed to connect to database");
    Ok(rb)
}