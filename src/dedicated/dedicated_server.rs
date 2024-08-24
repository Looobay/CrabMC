use crate::minecraft_server::IP;
use crate::network::server_connection_listener::start_tcp_server_listener;
use crate::{MC_GAME_VERSION, MC_PROTOCOL_VERSION};
use log::info;

pub fn init_dedicated_server(port: &str) {
    let ip = IP::new("0.0.0.0");

    info!(
        "Starting Minecraft server version {} and protocol version {}",
        MC_GAME_VERSION, MC_PROTOCOL_VERSION
    );

    info!("Loading properties");
    // TODO => Load properties from minecraft_server.rs

    let address = format!("{}:{}", ip.get_local_ip(), port);
    start_tcp_server_listener(address);

    // TODO => Add the authentication

    // TODO => Convert old user
}
