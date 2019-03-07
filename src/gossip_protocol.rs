//use std::process;
//use std::collections::HashSet;


use types::*;
use type_init::*;
use data_structure::*;
//use data_structure::*;

use jsonrpc_client_http::HttpTransport;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleGossipProtocol {
    pub list_neighbor: Vec<usize>
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


pub fn compute_gossip_protocol(common_init: CommonInit, address: String) -> GossipProtocol {
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



pub fn compute_simple_gossip_protocol(common_init: &CommonInit, address: String) -> SimpleGossipProtocol {
    let nb_reg = common_init.registrars.len();
    let mut the_vect = Vec::<usize>::new();
    for i_reg in 0..nb_reg {
        let addr_reg = &common_init.registrars[i_reg];
        if addr_reg.address != address {
            the_vect.push(i_reg);
        }
    }
    SimpleGossipProtocol { list_neighbor: the_vect }
}


jsonrpc_client!(pub struct InternalClient {
    pub fn internal_check(&mut self, transmission: String) -> RpcRequest<String>;
    pub fn registration_info(&mut self, request: String) -> RpcRequest<String>;
});

fn check_transaction_kernel(mesg: Message) -> String {
    let lnk : String = "http://".to_string() + &mesg.ip_plus_port;
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle(&lnk).unwrap();
    let mut client = InternalClient::new(transport_handle);
    let result1 = client.internal_check(mesg.message).call().unwrap();
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
            let ereq_str = serde_json::to_string(ereq).unwrap();
            for lnk_subscribed in &eval.list_subscribed_node {
                let transport = HttpTransport::new().standalone().unwrap();
                let transport_handle = transport.handle(&lnk_subscribed).unwrap();
                let mut client = InternalClient::new(transport_handle);
                let _result1 = client.registration_info(ereq_str.clone()).call().unwrap();
            }
        },
        None => {println!("send_info_to_registered error. The topic is missing which should not happen");},
    }
}





fn check_transaction(registrar: SingleRegistrar, ereq: &SumTypeRequest) -> bool {
    let str0 : String = registrar.ip_address[0].to_string();
    let str1 : String = registrar.ip_address[1].to_string();
    let str2 : String = registrar.ip_address[2].to_string();
    let str3 : String = registrar.ip_address[3].to_string();
    let str4 : String = registrar.port.to_string();
    let ip_plus_port : String = str0 + "." + &str1 + "." + &str2 + "." + &str3 + ":" + &str4;
    //
    let ereq_str = serde_json::to_string(ereq).unwrap();
    let mesg = Message { ip_plus_port: ip_plus_port, message: ereq_str };
    //
    let reply = check_transaction_kernel(mesg);
    println!("check_transaction result={}", reply);
    //
    let res : Result<MKBoperation,_> = serde_json::from_str(&reply);
    match res {
        Ok(eval) => {println!("check_transaction eval={:?}", eval); eval.result},
        Err(e) => {println!("check_transaction error e={}", e); false},
    }
}











pub fn check_mkb_operation(common_init: CommonInit, sgp: SimpleGossipProtocol, ereq: SumTypeRequest) -> bool {
    let nb_neigh = sgp.list_neighbor.len();
    let mut nb_true = 0;
    for i_neigh in 0..nb_neigh {
        let i_reg = sgp.list_neighbor[i_neigh];
        let eval = check_transaction(common_init.registrars[i_reg].clone(), &ereq.clone());
        if eval {
            nb_true = nb_true + 1
        }
    }
    let quot = (nb_true as f32) / (nb_neigh as f32);
    println!("check_mkb_operation, nb_true={} nb_neigh={} quot={}", nb_true, nb_neigh, quot);
    if quot > common_init.consensus_fraction {
        return true;
    }
    return false;
}
