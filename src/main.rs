mod auth;
mod dedicated;
mod eula;
mod math;
mod minecraft_server;
mod network;
mod server_properties;
mod util;

use crate::dedicated::dedicated_server::init_dedicated_server;
use crate::util::logger::{deleting_logs, logs_size, setup_logging};
use clap::{Arg, Command};
use colored::Colorize;
use eula::*;
use log::*;
use server_properties::*;
use std::thread;
use crate::util::update::update_program;

const MC_GAME_VERSION: &str = "1.20.2";
const MC_PROTOCOL_VERSION: i32 = 764;
const DESC: &str = "CrabMC ~ A rusty minecraft server";
const NAME: &str = "CrabMC";
const VERSION: &str = "1.0";
const AUTHOR: &str = "Looobay";

// ==================================
// The starting point of the program.
// ==================================
fn main() {
    match setup_logging() {
        Ok(_) => (),
        Err(e) => error!("Error with logger: {}", e),
    }
    info!(
        "\nThank you for using CrabMC !\n{}",
        "We are not affiliated with Mojang AB or Microsoft Corporation."
            .bold()
            .underline()
    );
    logs_size();

    // This will be added in the next release
    /*if let Err(e) = update_program() {
        eprintln!("Failed to update: {}", e);
    }*/

    server_setup();
}

// =========================================================================================================================
// It setup the parameters when you call the program from command lines. Start the dedicated server when the EULA is agreed.
// I recommend you to read this, and you will understand it, just take your time and all will be fine :).
// =========================================================================================================================
fn server_setup() {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESC)
        .arg(
            Arg::new("nogui")
                .long("nogui")
                .help("Start the server without GUI")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("initSettings")
                .long("initSettings")
                .help("Initializes 'server.properties' and 'eula.txt', then quits")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("demo")
                .long("demo")
                .help("Start the server in demo mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bonusChest")
                .long("bonusChest")
                .help("Start the server with a bonus chest in the world")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("forceUpgrade")
                .long("forceUpgrade")
                .help("Force the upgrade of a world")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("eraseCache")
                .long("eraseCache")
                .help("Delete the cache of the server")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("safeMode")
                .long("safeMode")
                .help("Loads level with vanilla data pack only")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(Arg::new("universe").long("universe"))
        .arg(Arg::new("world").long("world"))
        .arg(Arg::new("port").long("port").help("The port of the server"))
        .arg(Arg::new("serverId").long("serverId"))
        .arg(Arg::new("pidFile").long("pidFile").value_name("FILE"))
        .arg(
            Arg::new("remakeText")
                .long("remakeText")
                .help("Just overwrite over eula.txt and server.properties")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("clearLogs")
                .long("clearLogs")
                .help("Delete the logs")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(Arg::new("nonOptions").long("nonOptions"))
        .get_matches();

    // Call the help printing from Clap
    if matches.value_source("help").is_some() {
        Command::new(NAME)
            .version(VERSION)
            .author(AUTHOR)
            .about(DESC)
            .print_help()
            .expect("Error when help");
        std::process::exit(0);
    }

    if matches.get_flag("nogui") {
        info!("No GUI");
    } else {
        info!("TODO: Load GUI");
    }

    if matches.get_flag("clearLogs") {
        info!("Deleting logs...");
        deleting_logs();
        std::process::exit(1);
    }
    // Delete EULA and server.properties and recreate them.
    if matches.get_flag("remakeText") {
        info!("Remake eula.txt and server.properties");
        delete_eula();
        delete_server_properties();
        match has_agreed_to_eula() {
            // Check if the EULA is agreed, if not don't run and if the file doesn't exist: create one
            Ok(_) => (),
            Err(e) => {
                error!("Error while checking EULA: {}", e);
                std::process::exit(1);
            }
        }
        match check_server_properties() {
            // Check if server.properties exist, if not create the standard server.properties
            Ok(_) => (),
            Err(e) => {
                error!("Error while checking server.properties: {}", e);
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }
    //
    if matches.get_flag("initSettings") {
        match has_agreed_to_eula() {
            // Check if the EULA is agreed, if not don't run and if the file doesn't exist: create one
            Ok(_) => (),
            Err(e) => {
                error!("Error while checking EULA: {}", e);
                std::process::exit(1);
            }
        }
        match check_server_properties() {
            // Check if server.properties exist, if not create the standard server.properties
            Ok(_) => (),
            Err(e) => {
                error!("Error while checking server.properties: {}", e);
                std::process::exit(1);
            }
        }

        info!("Initialized eula.txt and server.properties !");
        std::process::exit(0);
    }
    // Here is when the dedicated server start with a custom port or with the standard Minecraft port
    if let Some(port) = matches.get_one::<String>("port") {
        let port = port.clone();

        let server_thread = thread::spawn(move || {
            match has_agreed_to_eula() {
                // Check if the EULA is agreed, if not don't run and if the file doesn't exist: create one
                Ok(_) => (),
                Err(e) => {
                    error!("Error while checking EULA: {}", e);
                    std::process::exit(1);
                }
            }
            match check_server_properties() {
                // Check if server.properties exist, if not create the standard server.properties
                Ok(_) => (),
                Err(e) => {
                    error!("Error while checking server.properties: {}", e);
                    std::process::exit(1);
                }
            }
            init_dedicated_server(&port);
        });
        server_thread
            .join()
            .expect("The server thread encountered an error");
    } else {
        let server_thread = thread::spawn(|| {
            match has_agreed_to_eula() {
                // Check if the EULA is agreed, if not don't run and if the file doesn't exist: create one
                Ok(_) => (),
                Err(e) => {
                    error!("Error while checking EULA: {}", e);
                    std::process::exit(1);
                }
            }
            match check_server_properties() {
                // Check if server.properties exist, if not create the standard server.properties
                Ok(_) => (),
                Err(e) => {
                    error!("Error while checking server.properties: {}", e);
                    std::process::exit(1);
                }
            }
            init_dedicated_server("25565");
        });
        match server_thread.join() {
            // Wait the thread to kill the program
            Ok(_) => (),
            Err(e) => {
                error!("The server thread encountered an error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
}
