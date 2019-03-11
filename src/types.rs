use std::process;
//use std::io;
//use serde::Deserialize;
use serde::*;
use type_init::*;
use chrono::prelude::*;
use std::collections::{HashMap, HashSet};
pub type HashType = Vec<u8>;


#[derive(Clone)]
pub struct MultihashType {
    pub val: multihash::Hash,
}


// Account and topics.

#[derive(Clone, Serialize, Deserialize)]
pub struct AccountCurrent {
    pub current_money: u64,
    pub data_current: String,
    pub hash: HashType,
    pub utc: DateTime<Utc>,
    pub nonce: u32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FullTopicData {
    pub topic_desc: TopicDescriptionEncode,
    pub list_active_reg: HashSet<String>,
    pub sgp: SimpleGossipProtocol,
    pub list_subscribed_node: HashSet<String>,
    pub all_account_state: HashMap<String,Vec<AccountCurrent>>
}

#[derive(Clone,Default)]
pub struct TopicAllInfo {
    pub all_topic_state: HashMap<String,FullTopicData>
}



#[derive(Serialize)]
pub struct SinglePairUserHash {
    pub name: String,
    pub hash: HashType,
}


#[derive(Serialize)]
pub struct ComputeHashOfTopic {
    pub topic: String,
    pub topic_desc: TopicDescriptionEncode,
    pub list_pair: Vec<SinglePairUserHash>,
}



// Gossip protocol

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SimpleGossipProtocol {
    pub list_neighbor: Vec<SingleRegistrarFinal>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingLine {
    pub list_direct_neighbor: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipProtocol {
    pub list_routing_line: Vec<RoutingLine>,
    pub initial_address: String
}




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

#[derive(Clone, Serialize, Deserialize)]
pub struct TopicDescriptionEncode {
    pub min_interval_insertion_micros: i64, // the number of allowed transactions per seconds. 0 for infinity
    pub capacity_mem: u32, // the total allowed capacity. If 0 for infinity
    pub retention_time: i64, // the retention policy of data. If 0, then not used.
    pub retention_size: u32, // the maximum number of versions are kept. If 0 then all are used.
    pub hash_method: MultihashType, // The hashing method used.
}

fn map_string_to_hash_meth(hash_method: String) -> Option<multihash::Hash> {
    match hash_method.as_ref() {
        "SHA1" => Some(multihash::Hash::SHA1),
        "SHA2256" => Some(multihash::Hash::SHA2256),
        "SHA2512" => Some(multihash::Hash::SHA2512),
        
        "SHA3512" => Some(multihash::Hash::SHA3512),
        "SHA3384" => Some(multihash::Hash::SHA3384),
        "SHA3256" => Some(multihash::Hash::SHA3256),
        "SHA3224" => Some(multihash::Hash::SHA3224),
        
        "Keccak224" => Some(multihash::Hash::Keccak224),
        "Keccak256" => Some(multihash::Hash::Keccak256),
        "Keccak384" => Some(multihash::Hash::Keccak384),
        "Keccak512" => Some(multihash::Hash::Keccak512),

        "Blake2b" => Some(multihash::Hash::Blake2b),
        "Blake2s" => Some(multihash::Hash::Blake2s),
        _ => None,
    }
}

fn map_hash_method_to_string(hash_meth: multihash::Hash) -> String {
    match hash_meth {
        multihash::Hash::SHA1 => "SHA1".to_string(),
        multihash::Hash::SHA2256 => "SHA2256".to_string(),
        multihash::Hash::SHA2512 => "SHA2512".to_string(),

        multihash::Hash::SHA3512 => "SHA3512".to_string(),
        multihash::Hash::SHA3384 => "SHA3384".to_string(),
        multihash::Hash::SHA3256 => "SHA3256".to_string(),
        multihash::Hash::SHA3224 => "SHA3224".to_string(),
            
        multihash::Hash::Keccak224 => "Keccak224".to_string(),
        multihash::Hash::Keccak256 => "Keccak256".to_string(),
        multihash::Hash::Keccak384 => "Keccak384".to_string(),
        multihash::Hash::Keccak512 => "Keccak512".to_string(),

        multihash::Hash::Blake2b => "Blake2b".to_string(),
        multihash::Hash::Blake2s => "Blake2s".to_string(),
    }
}

impl Serialize for MultihashType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        serializer.serialize_str(&map_hash_method_to_string(self.val))
    }
}



impl<'de> Deserialize<'de> for MultihashType {
    fn deserialize<D>(deserializer: D) -> Result<MultihashType, D::Error> where D: Deserializer<'de>,
    {
        let estr : String = String::deserialize(deserializer)?;
        match map_string_to_hash_meth(estr) {
            Some(eval) => Ok(MultihashType { val: eval}),
            None => Err(de::Error::custom("Cannot deserialize hash".to_string())),
        }
    }
}








pub fn get_topic_desc_encode(topic_desc: &TopicDescription) -> TopicDescriptionEncode {
    let hash_meth = match map_string_to_hash_meth(topic_desc.hash_method.clone()) {
        Some(eval) => {MultihashType { val: eval }},
        None => {
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


#[derive(Default, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ExportTopicInformation {
    pub min_interval_insertion_micros: i64, // the number of allowed transactions per seconds. 0 for infinity
    pub capacity_mem: u32, // the total allowed capacity. If 0 for infinity
    pub retention_time: i64, // the retention policy of data. If 0, then not used.
    pub retention_size: u32, // the maximum number of versions are kept. If 0 then all are used.
    pub one_registrar_ip_addr: Vec<u8>,
    pub one_registrar_port: u16
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
pub struct ListRegistrar {
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
pub struct RequestInfoTopic {
    pub topic: String,
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct InternalRequestTopicInfo {
    pub topic: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TopicExportation {
    pub topic: String,
    pub topic_info: FullTopicData,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RetrieveHashForVRF {
    pub topic: String,
}


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TrivialAnswer {
}


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AnswerHashForVRF {
    pub hash: String,
}







pub fn get_topic(ereq: &SumTypeRequest) -> Option<String> {
    use types::SumTypeRequest::*;
    match ereq {
        Accountinfo(eacct) => { Some(eacct.topic.clone()) },
        Depositrequest(edep) => { Some(edep.topic.clone()) },
	Paymentrequest(epay) => { Some(epay.topic.clone()) },
	Withdrawrequest(ewith) => { Some(ewith.topic.clone()) },
	Senddatarequest(esend) => { Some(esend.topic.clone()) },
        _ => None,
    }
}





#[derive(Clone, Serialize, Deserialize)]
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
    Internalrequesttopicinfo(InternalRequestTopicInfo),
    Fulltopicexport(TopicExportation),
    Retrievehashforvrf(RetrieveHashForVRF),
}



#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct MKBoperation {
    pub signature: Option<HashType>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SumTypeAnswer {
    Mkboperation(MKBoperation),
    Exporttopicinformation(ExportTopicInformation),
    Answerhashforvrf(AnswerHashForVRF),
    Trivialanswer(TrivialAnswer),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TypeAnswer {
    pub result: bool, 
    pub text: String,
    pub answer: SumTypeAnswer,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TypeAnswerComplete {
    pub result: bool, 
    pub text: String,
    pub signature: String,
    pub answer: SumTypeAnswer,
}

pub fn get_typeanswer_complete(ans: TypeAnswer, sign: String) -> TypeAnswerComplete {
    TypeAnswerComplete { result: ans.result, text: ans.text, signature: sign, answer: ans.answer}
}





#[derive(Clone, Serialize, Deserialize)]
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
pub struct MessageTrans {
    pub ip_plus_port: String,
//    pub sender: String,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageTransRed {
//    pub ip_plus_port: String,
//    pub sender: String,
    pub message: String,
}








#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedString {
    pub result: String,
    pub sig: Vec<u8>,
}
