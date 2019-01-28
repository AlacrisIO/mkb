//extern crate serde;
//extern crate serde_json;
//use std::vec;
//use self::serde::{Serialize, Serializer, Deserialize, Deserializer};
//use self::serde::{Deserialize, Serialize};
//use self::serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct SingleRegistrar {
    name: String,
    address: String,
    public_key: String,
    url: String,
    port: i32
}

#[derive(Serialize, Deserialize)]
pub struct CommonInit {
    registrars: Vec<SingleRegistrar>,
    consensus_fraction: f32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SingleEnt {
    name: String,
    address: String,
    public_key: String,
    private_key: String,
    password: String,
    pub database_file: String
}


    
