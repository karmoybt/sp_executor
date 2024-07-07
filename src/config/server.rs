use std::env;
use std::net::SocketAddr;

pub fn obtener_direccion_del_servidor() -> SocketAddr {
    env::var("DIRECCION_DEL_SERVIDOR")
        .expect("DIRECCION_DEL_SERVIDOR debe estar configurada")
        .parse::<SocketAddr>()
        .expect("Formato de dirección del servidor no válido")
}
