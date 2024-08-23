use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use log::{error, info, warn};
use crate::math::var_int_and_long::read_var_int;
use crate::MC_PROTOCOL_VERSION;
use crate::network::packet_handler::{packet_listener};
use crate::util::chrono::run_with_timeout;

pub fn start_tcp_server_listener(address: String) {
    let listener = TcpListener::bind(&address).expect("Failed to bind to address");
    info!("Starting Minecraft server on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_client(s);
                });
            }
            Err(e) => {
                error!("Error when starting TCP server listener: {}", e);
                warn!("Perhaps a server is already running on that port?");
                std::process::exit(1);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut data: Vec<u8> = vec![];
    let mut state: u8 = 0;
    let mut buffer = vec![0; 1000];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                data = buffer[..bytes_read].to_vec();
                packet_listener(data, &mut state, &mut stream);
            },
            Err(e) => {
                error!("Error: {}", e);
                break;
            }
        }
    }
}
