use std::error::Error;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;

pub fn generate_key_pair() -> Result<(RsaPrivateKey, RsaPublicKey), Box<dyn Error>> {
    let mut rng = OsRng; // Random from Operating System

    // Generate a key pair in 1024 bits
    let private_key = RsaPrivateKey::new(&mut rng, 1024)?;

    // The public key is produced from the private one
    let public_key = RsaPublicKey::from(&private_key);

    Ok((private_key, public_key))
}