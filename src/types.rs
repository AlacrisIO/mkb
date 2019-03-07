use std::process;
use serde::Deserialize;
pub type HashType = Vec<u8>;




// Internal types

// RPC request from the users

#[derive(Clone, Default, Hash, Serialize, Deserialize)]
pub struct TopicDescription {
    pub topic: String, // the name of the topic
    pub min_interval_insertion_micros: i64, // the number of allowed transactions per seconds. 0 for infinity
    pub capacity_mem: u32, // the total allowed capacity. If 0 for infinity
    pub retention_time: i64, // the retention policy of data. If 0, then not used.
    pub retention_size: u32, // the maximum number of versions are kept. If 0 then all are used.
    pub hash_method: String, // The hashing method used.
}

#[derive(Clone)]
pub struct TopicDescriptionEncode {
    pub min_interval_insertion_micros: i64, // the number of allowed transactions per seconds. 0 for infinity
    pub capacity_mem: u32, // the total allowed capacity. If 0 for infinity
    pub retention_time: i64, // the retention policy of data. If 0, then not used.
    pub retention_size: u32, // the maximum number of versions are kept. If 0 then all are used.
    pub hash_method: multihash::Hash, // The hashing method used.
}

pub fn get_topic_desc_encode(topic_desc: &TopicDescription) -> TopicDescriptionEncode {
    let hash_meth=match topic_desc.hash_method.as_ref() {
        "SHA1" => multihash::Hash::SHA1,
        "SHA2256" => multihash::Hash::SHA2256,
        "SHA2512" => multihash::Hash::SHA2512,
        
        "SHA3512" => multihash::Hash::SHA3512,
        "SHA3384" => multihash::Hash::SHA3384,
        "SHA3256" => multihash::Hash::SHA3256,
        "SHA3224" => multihash::Hash::SHA3224,
        
        "Keccak224" => multihash::Hash::Keccak224,
        "Keccak256" => multihash::Hash::Keccak256,
        "Keccak384" => multihash::Hash::Keccak384,
        "Keccak512" => multihash::Hash::Keccak512,

        "Blake2b" => multihash::Hash::Blake2b,
        "Blake2s" => multihash::Hash::Blake2s,
        _ => {
            println!("Non matching hash algorithm");
	    process::exit(1);
        },
    };
    TopicDescriptionEncode{min_interval_insertion_micros: topic_desc.min_interval_insertion_micros,
                           capacity_mem: topic_desc.capacity_mem,
                           retention_time: topic_desc.retention_time,
                           retention_size: topic_desc.retention_size,
                           hash_method: hash_meth}
}




#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct AccountInfo {
    pub topic: String,
    pub account_name: String,
    pub public_key: String
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
    pub hash: HashType,
    pub amount: u64
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct WithdrawRequest {
    pub topic: String,
    pub account_name: String,
    pub hash: HashType,
    pub amount: u64
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct SendDataRequest {
    pub topic: String,
    pub account_name: String,
    pub hash: HashType,
    pub data: String
}

// Queries on the database

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct GetInfoRequest {
    pub topic: String,
    pub account_name: String,
}





#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct AddSubscriber {
    pub topic: String,
    pub subscriber_name: String,
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct RemoveSubscriber {
    pub topic: String,
    pub subscriber_name: String,
}



#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct AddRegistrar {
    pub topic: String,
    pub registrar_name: String,
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct RemoveRegistrar {
    pub topic: String,
    pub registrar_name: String,
}





#[derive(Clone, Hash, Serialize, Deserialize)]
pub enum SumTypeRequest {
    Topiccreationrequest(TopicDescription),
    Accountinfo(AccountInfo),
    Depositrequest(DepositRequest),
    Paymentrequest(PaymentRequest),
    Withdrawrequest(WithdrawRequest),
    Senddatarequest(SendDataRequest),
    Addsubscriber(AddSubscriber),
    Removesubscriber(RemoveSubscriber),
    Addregistrar(AddRegistrar),
    Removeregistrar(RemoveRegistrar),
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




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MKBoperation {
    pub result: bool, 
    pub signature: Option<HashType>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedString {
    pub result: String,
    pub sig: String,
}

