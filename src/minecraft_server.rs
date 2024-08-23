pub struct IP{
    ip: String,
}

impl IP{
    pub fn new(ip:&str) -> Self{ // Set up the new ip
        IP {
            ip: ip.to_string(),
        }
    }

    pub fn get_local_ip(&self) -> &str { // Return the local ip stored in &self
        &self.ip
    }
}
/* Issue with RSA
pub fn initialize_key_pair(){
    info!("Generating keypair");

    match generate_key_pair(){
        Ok(_key_pair) => (),
        Err(e) => {
            error!("Failed to generate key pair: {}", e);
        }
    }
}
*/