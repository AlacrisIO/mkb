use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleRegistrar {
    pub name: String,
    pub address: String,
    pub public_key: String,
    pub ip_addr: Vec<u8>,
    pub port: u16,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct CommonInit {
    pub registrars: Vec<SingleRegistrar>,
    pub consensus_fraction: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleRegistrarFinal {
    pub name: String,
    pub address: String,
    pub public_key: secp256k1::key::PublicKey,
    pub ip_addr: Vec<u8>,
    pub port: u16,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct CommonInitFinal {
    pub registrars: Vec<SingleRegistrarFinal>,
    pub map_name_idx: HashMap<String,usize>,
    pub map_address_idx: HashMap<String,usize>,
    pub consensus_fraction: f32
}


#[derive(Clone, Serialize, Deserialize)]
pub struct SingleAddress {
    pub ip_addr: Vec<u8>,
    pub port: u16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListSingleAddress {
    pub list_sing_addr: Vec<SingleAddress>,
}

pub fn retrieve_complete_list_registrar(common_init: CommonInitFinal) -> ListSingleAddress {
    let mut e_vect = Vec::<SingleAddress>::new();
    for e_rec in common_init.registrars {
        let e_sing = SingleAddress { ip_addr: e_rec.ip_addr, port: e_rec.port};
        e_vect.push(e_sing);
    }
    ListSingleAddress { list_sing_addr: e_vect }
}




pub fn retrieve_common_init_final(common_init: &CommonInit) -> CommonInitFinal {
    let mut e_vect = Vec::<SingleRegistrarFinal>::new();
    let mut e_map_name = HashMap::<String,usize>::new();
    let mut e_map_address = HashMap::<String,usize>::new();
    let mut idx=0;
    for eval in common_init.registrars.clone() {
        let eval_b = SingleRegistrarFinal{name: eval.name.clone(), address: eval.address.clone(),
                                          public_key: retrieve_public_key(&eval.public_key.clone()),
                                          ip_addr: eval.ip_addr, port: eval.port};
        e_vect.push(eval_b);
        e_map_name.insert(eval.name.clone(), idx);
        e_map_address.insert(eval.address.clone(), idx);
        idx +=1;
    }
    CommonInitFinal{registrars: e_vect,
                    map_name_idx: e_map_name, map_address_idx: e_map_address,
                    consensus_fraction: common_init.consensus_fraction}
}

pub fn get_registrar_by_address(address: String, common_init: &CommonInitFinal) -> Option<SingleRegistrarFinal> {
    let iter = common_init.map_address_idx.get(&address);
    match iter {
        None => {
            println!("Failed to find entry in get_registrar_by_address, address={}", address);
            None},
        Some(eval) => Some(common_init.registrars[*eval].clone()),
    }
}

pub fn get_registrar_by_name(name: String, common_init: &CommonInitFinal) -> Option<SingleRegistrarFinal> {
    let iter = common_init.map_name_idx.get(&name);
    match iter {
        None => {
            println!("Failed to find entry in get_registrar_by_name, name={}", name);
            None},
        Some(eval) => Some(common_init.registrars[*eval].clone()),
    }
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

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .filter_map(|b| std::str::from_utf8(b).ok())
        .filter_map(|s| u8::from_str_radix(s, 16).ok())
        .collect()
}

pub fn bytes_to_hex(ev: Vec<u8>) -> String {
    hex::encode(ev)
}




pub fn retrieve_secret_key(ekey: &String) -> secp256k1::key::SecretKey {
    secp256k1::key::SecretKey::from_slice(&hex_to_bytes(ekey)).expect("Error in reading secret key")
}

pub fn retrieve_public_key(ekey: &String) -> secp256k1::key::PublicKey {
    secp256k1::key::PublicKey::from_slice(&hex_to_bytes(ekey)).expect("Error in reading public key")
}




pub fn get_localinit_final(local_init: &LocalInit) -> LocalInitFinal {
//    println!("local_init.secret_key={}", local_init.secret_key);
//    println!("local_init.public_key={}", local_init.public_key);
    let secret_key_nat : secp256k1::key::SecretKey = retrieve_secret_key(&local_init.secret_key.clone());
    let public_key_nat : secp256k1::key::PublicKey = retrieve_public_key(&local_init.public_key.clone());
    //
    LocalInitFinal {name: local_init.name.clone(), address: local_init.address.clone(),
                    public_key: public_key_nat, secret_key: secret_key_nat,
                    password: local_init.password.clone(), database_file: local_init.database_file.clone()}
}




pub fn read_common_init_ci<P: AsRef<Path>>(path: P) -> CommonInitFinal {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("Error in opening path");
    println!("read_common_init_ci : After open statement");

    let reader = BufReader::new(file);
    println!("read_common_init_ci : After reader statement");

    // Read the JSON contents of the file as an instance of `CommonInit`.
    let u : CommonInit = serde_json::from_reader(reader).expect("Error in parsing of input");
    println!("read_common_init_ci : We have read u");
    retrieve_common_init_final(&u)
}



pub fn read_local_init<P: AsRef<Path>>(path: P) -> LocalInitFinal {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("Error read_local_init, operation");
    let reader = BufReader::new(file);

    let u : LocalInit = serde_json::from_reader(reader).expect("Error reading types::LocalInit");
    get_localinit_final(&u)
}

