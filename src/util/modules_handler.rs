use log::{error, info};
use std::fs;
use std::path::Path;

//===========================================================
// You can load modules (rust files) in the server code base.
//===========================================================
pub fn _setup_modules() {
    let dir = Path::new("modules");

    if !dir.is_dir() {
        // Create the modules file
        if let Err(e) = fs::create_dir_all(dir) {
            error!("Error when creating folder: {}", e);
        }
    }
}
