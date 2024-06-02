use std::env;
use std::net::SocketAddr;

pub fn get_server_addr() -> SocketAddr {
    env::var("SERVER_ADDR")
        .expect("SERVER_ADDR must be set")
        .parse::<SocketAddr>()
        .expect("Invalid server address format")
}
