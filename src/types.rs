//extern crate serde;
//extern crate serde_json;
//use std::vec;
//use self::serde::{Serialize, Serializer, Deserialize, Deserializer};
//use self::serde::{Deserialize, Serialize};
//use self::serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleRegistrar {
    pub name: String,
    pub address: String,
    pub public_key: String,
    pub ip_address: Vec<u8>,
    pub port: u16
}

#[derive(Serialize, Deserialize)]
pub struct CommonInit {
    pub registrars: Vec<SingleRegistrar>,
    pub consensus_fraction: f32
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LocalInit {
    pub name: String,
    pub address: String,
    pub public_key: String,
    pub private_key: String,
    pub password: String,
    pub database_file: String
}

// RPC request from the users

#[derive(Serialize, Deserialize)]
pub struct AddAccountRequest {
    account_name: String,
    public_key: String,
    private_key: String
}

#[derive(Serialize, Deserialize)]
pub struct DepositRequest {
    account_name: String,
    amount: f32
}

#[derive(Serialize, Deserialize)]
pub struct PaymentRequest {
    account_name_sender: String,
    account_name_receiver: String,
    amount: f32
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawRequest {
    account_name: String,
    amount: f32
}

#[derive(Serialize, Deserialize)]
pub struct SendDataRequest {
    account_name: String,
    key: String,
    data: String
}

#[derive(Serialize, Deserialize)]
pub struct GetDataRequest {
    account_name: String,
    key: String
}

