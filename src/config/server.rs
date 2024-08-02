use std::env;
use std::net::SocketAddr;

pub fn obtener_direccion_del_servidor() -> SocketAddr {
    env::var("BACK")
        .expect("La Ruta BACK debe estar configurada")
        .parse::<SocketAddr>()
        .expect("Formato de dirección del servidor no válido")
}
// pub fn obtener_direccion_del_front() -> Result<SocketAddr, String> {
//     env::var("FRONT")
//         .map_err(|_| "La Ruta FRONT debe estar configurada".to_string())
//         .and_then(|addr_str| addr_str.parse::<SocketAddr>()
//             .map_err(|_| "Formato de dirección del servidor no válido".to_string()))
// }