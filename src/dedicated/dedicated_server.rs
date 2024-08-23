use std::error::Error;
use std::sync::Arc;
use std::thread;
use log::{error, info, warn};
use tiny_http::*;
use crate::{MC_GAME_VERSION, MC_PROTOCOL_VERSION};
use crate::minecraft_server::IP;
use crate::network::server_connection_listener::start_tcp_server_listener;

pub fn init_dedicated_server(port: &str) {
    let mut ip = IP::new("0.0.0.0");

    info!("Starting Minecraft server version {} and protocol version {}", MC_GAME_VERSION, MC_PROTOCOL_VERSION);

    info!("Loading properties");
    // TODO => Load properties from minecraft_server.rs

    let address = format!("{}:{}", ip.get_local_ip(), port);

    // Correctly handle the Result type
    start_tcp_server_listener(address);

    // TODO => Add the authentication

    // TODO => Convert old user
}
