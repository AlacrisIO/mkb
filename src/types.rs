//use std::process;
//use std::io;
//use serde::Deserialize;
use serde::*;
use type_init::*;
use type_hash::*;
use vrf::*;
use chrono::prelude::*;
use std::collections::{HashMap, HashSet};
pub type HashType = Vec<u8>;

// Account and topics.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountCurrent {
    pub current_money: u64,
    pub data_current: String,
    pub hash: HashType,
    pub utc: DateTime<Utc>,
    pub nonce: u32
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TopicDescription {
    pub topic: String, // the name of the topic
    pub committee_size: i32, // committee size. Can be 0 if no committee is computed.
    pub min_interval_insertion_micros: i64, // the number of allowed transactions per seconds. 0 for infinity
    pub total_capacity_mem: u64, // the total allowed capacity. If 0 for infinity
    pub instant_capacity_mem: u64, // the total allowed capacity. If 0 for infinity
    pub total_throughput_per_min: u64, //
    pub total_throughput_per_sec: u64, // 
    pub retention_time: i64, // the retention policy of data. If 0, then not used.
    pub retention_size: u32, // the maximum number of versions are kept. If 0 then all are kept.
    pub hash_method: MultihashType, // The hashing method used.
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TopicTest {
    pub topic: String, // the name of the topic
    pub committee_size_str: String, // committee size. Can be 0 if no committee is computed.
}




#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullTopicData {
    pub topic_desc: TopicDescription,
    pub list_active_reg: HashSet<String>,
    pub sgp: SimpleGossipProtocol,
    pub committee: Vec<String>,
    pub list_subscribed_node: HashSet<String>,
    pub all_account_state: HashMap<String,Vec<AccountCurrent>>
}

#[derive(Clone,Default)]
pub struct TopicAllInfo {
    pub all_topic_state: HashMap<String,FullTopicData>
}



#[derive(Serialize)]
pub struct SinglePairUserHash {
    pub account_name: String,
    pub hash: HashType,
}


#[derive(Serialize)]
pub struct ComputeHashOfTopic {
    pub topic: String,
    pub topic_desc: TopicDescription,
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


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ExportTopicInformation {
    pub topic_desc: TopicDescription,
    pub one_registrar_ip_addr: Vec<u8>,
    pub one_registrar_port: u16,
    pub list_registrar_name: Vec<String>
}





#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct AccountInfo {
    pub topic: String,
    pub account_name: String,
}






#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct DepositRequest {
    pub topic: String,
    pub account_name: String,
    pub hash: HashType,
    pub amount: u64
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub topic: String,
    pub account_name_sender: String,
    pub account_name_receiver: String,
    pub hash: HashType,
    pub amount: u64
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct WithdrawRequest {
    pub topic: String,
    pub account_name: String,
    pub hash: HashType,
    pub amount: u64
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct SendDataRequest {
    pub topic: String,
    pub account_name: String,
    pub hash: HashType,
    pub data: String
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct SendDataRequestInput {
    pub topic: String,
    pub account_name: String,
    pub hash: String,
    pub data: String
}



// Queries on the database

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct GetInfoRequest {
    pub topic: String,
    pub account_name: String,
}





#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct AddSubscriber {
    pub topic: String,
    pub subscriber_name: String,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct RemoveSubscriber {
    pub topic: String,
    pub subscriber_name: String,
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct TotalListRegistrar {
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct TopicListRegistrar {
    pub topic: String
}


#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct AddRegistrar {
    pub topic: String,
    pub registrar_address: String,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct AddRegistrarInput {
    pub topic: String,
    pub registrar_name: String,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct RemoveRegistrar {
    pub topic: String,
    pub registrar_address: String,
}


#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct RequestInfoTopic {
    pub topic: String,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct InternalRequestTopicInfo {
    pub topic: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicExportation {
    pub topic: String,
    pub topic_info: FullTopicData,
}


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TrivialAnswer {
}




#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct TripleRequest {
    pub topic: String,
    pub account_name: String,
    pub nonce: u32,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
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
    Setcommittee(Committee),
    Getlatestentry(GetInfoRequest),
    Triplerequest(TripleRequest),
}

pub fn get_topic_export_subscriber(ereq: &SumTypeRequest) -> Option<String> {
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

pub fn get_committee_size(top_desc: &TopicDescription, esumreq: SumTypeRequest) -> i32 {
    use types::SumTypeRequest::*;
    match esumreq {
        Addregistrar(_eval) => {
            return top_desc.committee_size;
        },
        Removeregistrar(_eval) => {
            return top_desc.committee_size;
        },
        _ => -1,
    }
}








#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct MKBoperation {
    pub signature: Option<HashType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SumTypeAnswer {
    Mkboperation(MKBoperation),
    Exporttopicinformation(ExportTopicInformation),
    Answerhashforvrf(AnswerHashForVRF),
    Accounttriplerequest(AccountCurrent),
    Accountlatestrequest(AccountCurrent),
    Trivialanswer(TrivialAnswer),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAnswer {
    pub result: bool, 
    pub text: String,
    pub answer: SumTypeAnswer,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct NoTopicOper {
    pub constness: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TopicOper {
    pub topic: String,
    pub constness: bool,
}


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum GossipOperationKind {
    Globalgossip(), // sending to all the other registrars
    Topicgossip(String), // sending to the registrars of the topic
    Nogossip(), // no sending to other registrars
}



pub fn get_topic_symbolic(ereq: &SumTypeRequest) -> GossipOperationKind {
    use types::SumTypeRequest::*;
    match ereq {
        Topiccreationrequest(_) => { GossipOperationKind::Nogossip()},
        Getlatestentry(_) => { GossipOperationKind::Nogossip()},
        Triplerequest(_) => { GossipOperationKind::Nogossip()},
        Accountinfo(eacct) => { GossipOperationKind::Topicgossip(eacct.topic.clone()) },
        Depositrequest(edep) => { GossipOperationKind::Topicgossip(edep.topic.clone()) },
        Paymentrequest(epay) => { GossipOperationKind::Topicgossip(epay.topic.clone()) },
        Withdrawrequest(ewith) => { GossipOperationKind::Topicgossip(ewith.topic.clone()) },
        Senddatarequest(esend) => { GossipOperationKind::Topicgossip(esend.topic.clone()) },
        Addsubscriber(eadd) => { GossipOperationKind::Topicgossip(eadd.topic.clone()) },
        Removesubscriber(erem) => { GossipOperationKind::Topicgossip(erem.topic.clone()) },
        Addregistrar(eadd) => { GossipOperationKind::Topicgossip(eadd.topic.clone()) },
        Removeregistrar(erem) => { GossipOperationKind::Topicgossip(erem.topic.clone()) },
        Internalrequesttopicinfo(ereq) => { GossipOperationKind::Topicgossip(ereq.topic.clone()) },
        Fulltopicexport(efte) => { GossipOperationKind::Topicgossip(efte.topic.clone()) },
        Retrievehashforvrf(erhfv) => { GossipOperationKind::Topicgossip(erhfv.topic.clone()) },
        Setcommittee(esc) =>  { GossipOperationKind::Topicgossip(esc.topic.clone()) },
    }
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
    pub address: String,
//    pub sender: String,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageTransRed {
//    pub ip_plus_port: String,
//    pub sender: String,
    pub message: String,
}
