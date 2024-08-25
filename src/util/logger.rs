use chrono::Local;
use colored::Colorize;
use fern::Dispatch;
use log::{error, info, warn};
use std::error;
use std::fs::OpenOptions;
use std::path::Path;
use std::{fs, io};

// ==========================================================
// Setup the logging system ; it also saves the logs in files.
// ==========================================================
pub fn setup_logging() -> Result<(), Box<dyn error::Error>> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let dir = Path::new("logs");

    // Delete the latest log file if it exists
    if let Err(e) = fs::remove_file("logs/latest.log") {
        if e.kind() != io::ErrorKind::NotFound {
            error!("Error when deleting file: {}", e);
        }
    }

    if !dir.is_dir() {
        // Create the logs directory if it does not exist
        if let Err(e) = fs::create_dir_all("logs") {
            error!("Error when creating folder: {}", e);
        }
    }

    let log_file_name = "logs/latest.log";

    // Determine the name for the compressed log file
    let mut log_num = 1;
    let compressed_log_file_name;
    loop {
        let path = format!("logs/{}-{}.log.gz", date, log_num);
        if !Path::new(&path).exists() {
            compressed_log_file_name = path;
            break;
        }
        log_num += 1;
    }

    // Set up logging
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(log_file_name)?;
    let compress_log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&compressed_log_file_name)?;

    let console_dispatch = Dispatch::new()
        .format(|out, message, record| {
            let level = record.level();
            let color_level = match level {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".white(),
            };
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                color_level,
                record.target(),
                message
            ))
        })
        .chain(io::stdout());

    let file_dispatch = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(log_file)
        .chain(compress_log_file);

    Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(console_dispatch)
        .chain(file_dispatch)
        .apply()?;

    Ok(())
}

// =====================
// Just delete the logs.
// =====================
pub fn deleting_logs() {
    if let Err(e) = fs::remove_dir_all("logs") {
        error!("Error when deleting logs: {}", e);
    } else {
        info!("Logs deleted");
    }
}

// ===============================================
// It just prints the size of the "logs" directory.
// ===============================================
pub fn logs_size() {
    let path = "logs";
    let mut dir_size = 0;

    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(e) => {
            error!("Error when reading directory: {}", e);
            return;
        }
    };

    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!("Error when reading an entry: {}", e);
                continue;
            }
        };

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(e) => {
                error!("Error when reading metadata: {}", e);
                continue;
            }
        };

        if metadata.is_file() {
            dir_size += metadata.len();
        } else if metadata.is_dir() {
            let lil_dir = match fs::read_dir(entry.path()) {
                Ok(sous_dir) => sous_dir,
                Err(e) => {
                    error!("Error when reading directory inside directory: {}", e);
                    continue;
                }
            };

            for lil_entry in lil_dir {
                let lil_entry = match lil_entry {
                    Ok(sous_entry) => sous_entry,
                    Err(e) => {
                        error!(
                            "Error when reading entry of directory inside directory: {}",
                            e
                        );
                        continue;
                    }
                };

                let lil_metadata = match lil_entry.metadata() {
                    Ok(sous_metadata) => sous_metadata,
                    Err(e) => {
                        error!(
                            "Error when getting metadata of directory inside directory: {}",
                            e
                        );
                        continue;
                    }
                };

                if lil_metadata.is_file() {
                    dir_size += lil_metadata.len();
                }
            }
        }
    }

    if dir_size > 10_000_000 {
        info!("Logs size: {} octets or {} bits", dir_size, dir_size * 8);
        warn!("{}", "The logs size is bigger than 10 megabytes!".yellow());
        warn!(
            "{}{}{}",
            "This is really big for logs so be careful and remember you can delete logs with "
                .yellow(),
            "--clearLogs".green().bold(),
            "!".yellow()
        );
    } else {
        info!("Logs size: {} octets or {} bits", dir_size, dir_size * 8);
    }
}
