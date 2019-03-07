use serde::Deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleRegistrar {
    pub name: String,
    pub address: String,
    pub public_key: String,
    pub ip_address: Vec<u8>,
    pub port: u16,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct CommonInit {
    pub registrars: Vec<SingleRegistrar>,
    pub consensus_fraction: f32
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalInit {
    pub name: String,
    pub address: String,
    pub public_key: String,
    pub secret_key: String,
    pub password: String,
    pub database_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalInitFinal {
    pub name: String,
    pub address: String,
    pub public_key: secp256k1::key::PublicKey,
    pub secret_key: secp256k1::key::SecretKey,
    pub password: String,
    pub database_file: String,
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .filter_map(|b| std::str::from_utf8(b).ok())
        .filter_map(|s| u8::from_str_radix(s, 16).ok())
        .collect()
}


pub fn retrieve_secret_key(ekey: &String) -> secp256k1::key::SecretKey {
    secp256k1::key::SecretKey::from_slice(&hex_to_bytes(ekey)).expect("Error in reading secret key")
}

pub fn retrieve_public_key(ekey: &String) -> secp256k1::key::PublicKey {
    secp256k1::key::PublicKey::from_slice(&hex_to_bytes(ekey)).expect("Error in reading public key")
}




pub fn get_localinit_final(local_init: &LocalInit) -> LocalInitFinal {
    println!("local_init.secret_key={}", local_init.secret_key);
    println!("local_init.public_key={}", local_init.public_key);
    let secret_key_nat : secp256k1::key::SecretKey = retrieve_secret_key(&local_init.secret_key.clone());
    let public_key_nat : secp256k1::key::PublicKey = retrieve_public_key(&local_init.public_key.clone());
    //
    LocalInitFinal {name: local_init.name.clone(), address: local_init.address.clone(),
                    public_key: public_key_nat, secret_key: secret_key_nat,
                    password: local_init.password.clone(), database_file: local_init.database_file.clone()}
}

