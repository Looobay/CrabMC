//===============================================
// This is a test for a rolling release system...
//===============================================
fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("Looobay")
        .repo_name("CrabMC")
        .bin_name("CrabMC")
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .build()?
        .update()?;

    if status.updated() {
        println!("The program has been updated to version {}", status.version());
    } else {
        println!("The program is already up-to-date.");
    }

    Ok(())
}