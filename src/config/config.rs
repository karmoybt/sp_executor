use rbatis::rbatis;
use std::env;

pub async fn init_rbatis() -> Result<Rbatis, String> {
    let rb = Rbatis::new();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL Variable no declarada");
    rb.link(&database_url).await.map_err(|e| e.to_string())?;
    Ok(rb)
}
