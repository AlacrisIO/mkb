//use std::vec;

//use std::collections::HashMap;

//use merkle_cbt::merkle_tree::CBMT;
//use merkle_cbt::Merge;

//use num_bigint::BigUint;
//use data_structure;


use serde::Deserialize;
//use numext_fixed_hash::H256;


pub type HashType = Vec<u8>;


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
    pub database_file: String
}

// Internal types

// RPC request from the users

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub struct AccountInfo {
    pub topic: String,
    pub account_name: String,
    pub public_key: String,
    pub secret_key: String
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct TopicCreationRequest {
    pub topic: String,
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct DepositRequest {
    pub topic: String,
    pub account_name: String,
    pub hash: HashType,
    pub amount: u64
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub topic: String,
    pub account_name_sender: String,
    pub account_name_receiver: String,
    pub amount: u64
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct WithdrawRequest {
    pub topic: String,
    pub account_name: String,
    pub amount: u64
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct SendDataRequest {
    pub topic: String,
    pub account_name: String,
    pub data: String
}

// Queries on the database

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct GetInfoRequest {
    pub topic: String,
    pub account_name: String,
}




#[derive(Clone, Hash, Serialize, Deserialize)]
pub enum SumTypeRequest {
    Topiccreationrequest(TopicCreationRequest),
    Accountinfo(AccountInfo),
    Depositrequest(DepositRequest),
    Paymentrequest(PaymentRequest),
    Withdrawrequest(WithdrawRequest),
    Senddatarequest(SendDataRequest),
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct ContainerTypeForHash {
    pub hash: HashType,
    pub esum: SumTypeRequest,
}



#[derive(Clone, Serialize, Deserialize)]
pub struct TransmissionRequest {
    pub address_origin: String,
    pub address_target: String,
    pub sum_type_request: SumTypeRequest,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    pub ip_plus_port: String,
//    pub sender: String,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageRed {
//    pub ip_plus_port: String,
//    pub sender: String,
    pub message: String,
}




#[derive(Clone, Serialize, Deserialize)]
pub struct MerkleVerification {
    pub result: bool, 
    pub signature: Option<HashType>,
}

