//extern crate serde;
//extern crate serde_json;
use std::vec;
//use self::serde::{Serialize, Serializer, Deserialize, Deserializer};
//use self::serde::{Deserialize, Serialize};
//use self::serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct SingleRegistrar {
    name: String,
    address: String,
    public_key: String,
    port: i32
}

#[derive(Serialize, Deserialize)]
struct CommonInit {
    registrars: Vec<SingleRegistrar>
}


struct SingleEnt {
    name: String,
    address: String,
    public_key: String,
    private_key: String,
    password: String
}


    
