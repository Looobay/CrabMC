use std::env;
use std::error::Error;
use log::info;

//================
// Just tryin smth
//================
pub fn update_program() -> Result<(), Box<dyn Error>> {
    let target = format!("{}-{}", env::consts::ARCH, env::consts::OS);

    let status = self_update::backends::github::Update::configure()
        .repo_owner("Looobay")
        .repo_name("CrabMC")
        .bin_name("CrabMC")
        .target(&target)
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .build()?
        .update()?;

    if status.updated() {
        info!("The program has been updated to version {}", status.version());
    } else {
        info!("The program is already up-to-date.");
    }

    Ok(())
}