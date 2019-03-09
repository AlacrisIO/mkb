//use std::process;
use std::collections::HashSet;


use types::*;
use type_init::*;
use data_structure::*;
use secp256k1::{Secp256k1, Message};
use jsonrpc_client_http::HttpTransport;


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


pub fn compute_gossip_protocol(common_init: CommonInitFinal, address: String) -> GossipProtocol {
    let len = address.len();
    let mut the_vect = Vec::new();
    for i in 0..len {
        let esub_str = &address[0..i];
        let get_routing_line = || {
            for eval in common_init.clone().registrars {
                if address != eval.address {
                    let eval_str = &eval.address[0..i];
                    if eval_str == esub_str {
                        return vec![eval.clone().address];
                    }
                }
            }
            vec![]
        };
        let e_line: RoutingLine = RoutingLine { list_direct_neighbor: get_routing_line() };
        the_vect.push(e_line);
        
    }
    GossipProtocol { list_routing_line: the_vect, initial_address: address }
}



pub fn compute_simple_gossip_protocol(common_init: &CommonInitFinal, address: String) -> SimpleGossipProtocol {
    let nb_reg = common_init.registrars.len();
    let mut the_vect = Vec::<SingleRegistrarFinal>::new();
    for i_reg in 0..nb_reg {
        let addr_reg = common_init.registrars[i_reg].clone();
        if addr_reg.address != address {
            the_vect.push(addr_reg);
        }
    }
    SimpleGossipProtocol { list_neighbor: the_vect }
}

pub fn compute_simple_gossip_protocol_topic(common_init: &CommonInitFinal, address: String, list_active_reg: HashSet<String>) -> SimpleGossipProtocol {
    let mut list_reg = Vec::<SingleRegistrarFinal>::new();
    for e_reg_addr in list_active_reg {
        if e_reg_addr != address {
            match get_registrar_by_address(e_reg_addr, common_init) {
                Some(e_reg) => list_reg.push(e_reg),
                None => {},
            }
        }
    }
    SimpleGossipProtocol { list_neighbor: list_reg }
}


pub fn get_ip_plus_port(ip_addr: Vec<u8>, port: u16) -> String {
    let str0 : String = ip_addr[0].to_string();
    let str1 : String = ip_addr[1].to_string();
    let str2 : String = ip_addr[2].to_string();
    let str3 : String = ip_addr[3].to_string();
    let str4 : String = port.to_string();
    let ip_plus_port : String = str0 + "." + &str1 + "." + &str2 + "." + &str3 + ":" + &str4;
    ip_plus_port    
}

jsonrpc_client!(pub struct InternalClient {
    pub fn internal_check(&mut self, transmission: String) -> RpcRequest<String>;
    pub fn registration_info(&mut self, request: String) -> RpcRequest<String>;
    pub fn internal_request_for_topic_info(&mut self, request: String) -> RpcRequest<String>;
});

fn check_transaction_kernel(mesg: MessageTrans) -> String {
    println!("check_transaction_kernel, step 1");
    let lnk : String = "http://".to_string() + &mesg.ip_plus_port;
    println!("check_transaction_kernel, step 2");
    let transport = HttpTransport::new().standalone().expect("Error in creation of HttpTransport");
    println!("check_transaction_kernel, step 3");
    let transport_handle = transport.handle(&lnk).expect("Error in creation of transport_handle");
    println!("check_transaction_kernel, step 4");
    let mut client = InternalClient::new(transport_handle);
    println!("check_transaction_kernel, step 5");
    let result1 = client.internal_check(mesg.message).call().expect("Error in calls of internal_check");
    println!("check_transaction_kernel, step 6");
    result1
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


pub fn send_info_to_registered(mut w_mkb: std::sync::MutexGuard<TopicAllInfo>, etopic: &String, ereq: &SumTypeRequest) {
    let x = (*w_mkb).all_topic_state.get_mut(etopic);
    match x {
        Some(eval) => {
            let ereq_str = serde_json::to_string(ereq).expect("Error in creation of ereq_str");
            for lnk_subscribed in &eval.list_subscribed_node {
                let transport = HttpTransport::new().standalone().expect("Error in creation of transport");
                let transport_handle = transport.handle(&lnk_subscribed).expect("Error in creation of transport_handle");
                let mut client = InternalClient::new(transport_handle);
                let _result1 = client.registration_info(ereq_str.clone()).call().expect("Error in creation of _result1");
            }
        },
        None => {println!("send_info_to_registered error. The topic is missing which should not happen");},
    }
}




fn check_transaction(registrar: SingleRegistrarFinal, ereq: &SumTypeRequest) -> bool {
    println!("check_transaction, step 1");
    let ip_plus_port = get_ip_plus_port(registrar.ip_addr, registrar.port);
    println!("check_transaction, step 2");
    //
    let ereq_str = serde_json::to_string(ereq).expect("Errot in creation of ereq_str");
    println!("check_transaction, step 3");
    let mesg = MessageTrans { ip_plus_port: ip_plus_port, message: ereq_str };
    println!("check_transaction, step 4");
    //
    let reply = check_transaction_kernel(mesg);
    println!("check_transaction result={}", reply);
    //
    let reply_b : SignedString = serde_json::from_str(&reply).expect("Error in signedstring");
    println!("check_transaction, step 5, reply_b.result={}", reply_b.result);
    let estr_u8 : &[u8] = reply_b.result.as_bytes();
    println!("check_transaction, step 6");
    let estr_u8_b = get_vector_len_thirtytwo(estr_u8);
    println!("check_transaction, step 7, estr_u8_b={:?}", estr_u8_b);
    let message = Message::from_slice(&estr_u8_b).expect("check_transaction : Error in creation of message");
    println!("check_transaction, step 8");
    let secp = Secp256k1::new();
    println!("check_transaction, step 9 reply_b.sig={:?}", reply_b.sig);
    println!("check_transaction, step 9, |reply_b.sig|={}", reply_b.sig.len());
    let esign : secp256k1::Signature = secp256k1::Signature::from_der(&reply_b.sig).expect("check_transaction : Error in extraction of signature");
    println!("check_transaction, step 10");
    let test : bool = secp.verify(&message, &esign, &registrar.public_key).is_ok();
    println!("check_transaction, step 11, test={}", test);
    if test==false {
        println!("check_transaction error in the verification");
        return false;
    }
    println!("check_transaction, step 12");
    //
    let res : Result<MKBoperation,_> = serde_json::from_str(&reply_b.result);
    println!("check_transaction, step 13");
    match res {
        Ok(eval) => {println!("check_transaction eval={:?}", eval); eval.result},
        Err(e) => {println!("check_transaction error e={}", e); false},
    }
}






pub fn get_topic_info_sgp_kernel(sgp: SimpleGossipProtocol, topic: String) -> Option<ExportTopicInformation> {
    for e_reg in sgp.list_neighbor {
        println!("get_topic_info_sgp, step 1");
        let lnk : String = "http://".to_string() + &get_ip_plus_port(e_reg.ip_addr, e_reg.port);
        println!("get_topic_info_sgp, step 2");
        let transport = HttpTransport::new().standalone().expect("Error in creation of HttpTransport");
        println!("get_topic_info_sgp, step 3");
        let transport_handle = transport.handle(&lnk).expect("Error in creation of transport_handle");
        println!("get_topic_info_sgp, step 4");
        let mut client = InternalClient::new(transport_handle);
        println!("get_topic_info_sgp, step 5");
        match client.internal_request_for_topic_info(topic.clone()).call() {
            Err(_) => {},
            Ok(eval) => {
                let result1_b : ExportTopicInformation = serde_json::from_str(&eval).expect("Error extracting the string");
                return Some(result1_b);
            }
        }
    }
    None
}







pub fn check_mkb_operation(common_init: CommonInitFinal, sgp: SimpleGossipProtocol, ereq: SumTypeRequest) -> bool {
    let nb_neigh = sgp.list_neighbor.len();
    let mut nb_true = 1; // because the main registrar is ok with that.
    for e_reg in sgp.list_neighbor {
        let eval = check_transaction(e_reg, &ereq.clone());
        if eval {
            nb_true = nb_true + 1
        }
    }
    let nb_total_reg = nb_neigh + 1;
    let quot = (nb_true as f32) / (nb_total_reg as f32);
    println!("check_mkb_operation, nb_true={} nb_neigh={} quot={}", nb_true, nb_neigh, quot);
    if quot > common_init.consensus_fraction {
        return true;
    }
    return false;
}


pub fn get_vector_len_thirtytwo(v: &[u8]) -> Vec<u8> {
    let len = v.len();
    let mut vret = vec![0x00; 32];
    let mut pos = 0;
    for i in 0..len {
	vret[pos] += v[i];
	pos += 1;
	if pos==32 {
            pos=0;
	}
    }
    vret
}
