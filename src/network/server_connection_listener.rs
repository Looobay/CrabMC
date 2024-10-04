use crate::network::packet_handler::packet_listener;
use log::{error, info, warn};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn start_tcp_server_listener(address: String) {
    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(e) => {
            error!("Error when starting TCP server listener: {}", e);
            warn!("Perhaps a server is already running on that port?");
            std::process::exit(1);
        }
    };

    info!("Starting Minecraft server on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_client(s);
                });
            }
            Err(e) => {
                error!("Error when listening: {}", e);
                std::process::exit(1);
            }
        }
    }
}

// Ok dude this function is fire
fn handle_client(mut stream: TcpStream) {
    let mut state: u8 = 0;
    let mut buffer = vec![0; 1000];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let data = buffer[..bytes_read].to_vec();
                packet_listener(data, &mut state, &mut stream);
            }
            Err(e) => {
                error!("Error: {}", e);
                break;
            }
        }
    }
}
