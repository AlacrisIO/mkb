//extern crate serde;
//extern crate serde_json;
//use std::vec;


//use merkle_cbt::merkle_tree::CBMT;
use merkle_cbt::Merge;

use num_bigint::BigUint;
//use serde_json::Number;
use serde::Deserialize;
//use serde_json::Value;




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleRegistrar {
    pub name: String,
    pub address: String,
    pub public_key: String,
    pub ip_address: Vec<u8>,
    pub port: u16
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
    pub private_key: String,
    pub password: String,
    pub database_file: String
}

// Internal types

// RPC request from the users

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    account_name: String,
    public_key: String,
    private_key: String
}

impl Merge for AccountInfo {
    fn merge(left: &Self, right: &Self) -> Self {
        right.clone()
    }
}



#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    account_info: AccountInfo,
    available_funds: BigUint
//    available_funds: u64
}


#[derive(Clone, Serialize, Deserialize)]
pub struct DepositRequest {
    account_name: String,
    amount: f32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    account_name_sender: String,
    account_name_receiver: String,
    amount: f32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawRequest {
    account_name: String,
    amount: f32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendDataRequest {
    account_name: String,
    key: String,
    data: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetDataRequest {
    account_name: String,
    key: String
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SumTypeRequest {
    Accountinfo(AccountInfo),
    Depositrequest(DepositRequest),
    Paymentrequest(PaymentRequest),
    Withdrawrequest(WithdrawRequest),
    Senddatarequest(SendDataRequest),
    Getdatarequest(GetDataRequest),
}



#[derive(Clone, Serialize, Deserialize)]
pub struct TransmissionRequest {
    pub address_origin: String,
    pub address_target: String,
    pub sum_type_request: SumTypeRequest,
}


