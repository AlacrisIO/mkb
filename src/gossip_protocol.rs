//use std::process;
use std::collections::HashSet;

use types::*;
use types::SumTypeAnswer::*;
use types::SumTypeRequest::*;
use type_init::*;
use type_sign::*;
use data_structure::*;
use jsonrpc_client_http::HttpTransport;



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


pub fn get_ip_plus_port(ip_addr: &Vec<u8>, port: u16) -> String {
    let str0 : String = ip_addr[0].to_string();
    let str1 : String = ip_addr[1].to_string();
    let str2 : String = ip_addr[2].to_string();
    let str3 : String = ip_addr[3].to_string();
    let str4 : String = port.to_string();
    let ip_plus_port : String = str0 + "." + &str1 + "." + &str2 + "." + &str3 + ":" + &str4;
    ip_plus_port    
}

jsonrpc_client!(pub struct InternalClient {
    pub fn internal_operation(&mut self, transmission: String) -> RpcRequest<String>;
    pub fn registration_info(&mut self, request: String) -> RpcRequest<String>;
});

fn send_transaction_kernel(mesg: MessageTrans) -> String {
//    println!("send_transaction_kernel, step 1");
    let lnk : String = "http://".to_string() + &mesg.ip_plus_port;
//    println!("send_transaction_kernel, step 2");
    let transport = HttpTransport::new().standalone().expect("Error in creation of HttpTransport");
//    println!("send_transaction_kernel, step 3");
    let transport_handle = transport.handle(&lnk).expect("Error in creation of transport_handle");
//    println!("send_transaction_kernel, step 4");
    let mut client = InternalClient::new(transport_handle);
//    println!("send_transaction_kernel, step 5");
    let result1 = client.internal_operation(mesg.message).call().expect("Error in calls of internal_check");
//    println!("send_transaction_kernel, step 6");
    result1
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




pub fn send_transaction(registrar: &SingleRegistrarFinal, esumreq: &SumTypeRequest) -> Option<TypeAnswer> {
    let ip_plus_port = get_ip_plus_port(&registrar.ip_addr, registrar.port);
    //
    let esumreq_str = serde_json::to_string(esumreq).expect("Errot in creation of esumreq_str");
    let mesg = MessageTrans { ip_plus_port: ip_plus_port, message: esumreq_str };
    //
    let reply = send_transaction_kernel(mesg);
    //
    let reply_b : SignedString = serde_json::from_str(&reply).expect("Error in signedstring");
    if check_signature_oper(registrar.public_key, &reply_b)==false {
        println!("send_transaction: error in the verification of signature");
        return None;
    }
    //
    let res : Result<TypeAnswer,_> = serde_json::from_str(&reply_b.result);
    match res {
        Ok(ans) => {println!("send_transaction: parsing success ans={:?}", ans);
                    Some(ans)},
        Err(e) => {println!("send_transaction: parsing error e={}", e); None},
    }
}






pub fn get_topic_info_sgp_kernel(sgp: SimpleGossipProtocol, topic: String) -> Option<ExportTopicInformation> {
    for e_reg in sgp.list_neighbor {
        let eval = InternalRequestTopicInfo { topic: topic.clone()};
        let esumreq = SumTypeRequest::Internalrequesttopicinfo(eval);
        let reply = send_transaction(&e_reg, &esumreq);
        match reply {
            None => {},
            Some(eval) => {
                if eval.result {
                    match eval.answer {
                        Exporttopicinformation(eval_b) => {
                            return Some(eval_b);
                        },
                        _ => {
                            println!("That case should not happen at all");
                        },
                    }
                }
            }
        }
    }
    None
}







pub fn check_mkb_operation(common_init: CommonInitFinal, sgp: SimpleGossipProtocol, esumreq: SumTypeRequest) -> bool {
    let nb_neigh = sgp.list_neighbor.len();
    let mut nb_true = 1; // because the main registrar is ok with that.
    for e_reg in sgp.list_neighbor {
        let eval = send_transaction(&e_reg, &esumreq.clone());
        match eval {
            None => {},
            Some(_) => {nb_true = nb_true + 1},
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

pub fn get_serialization_typeanswer(e_ans: TypeAnswer) -> String {
    match e_ans.result {
        false => {
            return "answer is false".to_string();
        },
        true => {
            match e_ans.answer {
                Trivialanswer(_eval) => {
                    return "successful answer, nothing to report".to_string();
                },
                _ => {},
            }
            let estr = serde_json::to_string(&e_ans.answer).expect("The serialization failed");
            estr
        },
    }
}


pub fn process_request_kernel(w_mkb: &mut std::sync::MutexGuard<TopicAllInfo>, my_reg: &SingleRegistrarFinal, esumreq: SumTypeRequest, sgp: SimpleGossipProtocol, common_init: CommonInitFinal) -> Result<TypeAnswer,String> {
    //
    // The Add registrar require a specfic operation of sending data.
    //
    match esumreq.clone() {
        Addregistrar(eadd) => {
            let reg_send_opt = get_registrar_by_address(eadd.registrar_name, &common_init);
            match reg_send_opt {
                None => {
                    return Err("Registrar is missing".to_string());
                },
                Some(reg_send) => {
                    let eval = (*w_mkb).all_topic_state.get(&eadd.topic);
                    match eval {
                        None => {
                            return Err("Error: Failed to find topic".to_string());
                        },
                        Some(eval_b) => {
                            let etopexport = TopicExportation { topic: eadd.topic, topic_info: eval_b.clone()};
                            let esumreq_b = SumTypeRequest::Fulltopicexport(etopexport);
                            let ans_opt = send_transaction(&reg_send, &esumreq_b);
                            match ans_opt {
                                None => {
                                    return Err("Error operation".to_string());
                                },
                                Some(_ans) => {
                                    print!("Sending data went ok");
                                },
                            }
                        },
                    }
                },
            }
            
        },
        _ => {},
    }
    //
    // The other operations
    //
    let res_oper = process_operation(w_mkb, common_init.clone(), my_reg, esumreq.clone());
    println!("process_request, step 4");
    if res_oper.result == false {
        return Err(res_oper.text);
    }
    match get_topic_symbolic(&esumreq.clone()) {
        GossipOperationKind::Nogossip() => {
            println!("GossipOperation::Nogossip, so nothing happens");
            Ok(res_oper)
        },
        GossipOperationKind::Topicgossip(eval) => {
            let x = (*w_mkb).all_topic_state.get(&eval);
            match x {
                None => {
                    return Err("Topic missing (but maybe error should be detected earlier)".to_string());
                },
                Some(eval) => {
                    println!("process_request, step 5");
                    let test = check_mkb_operation(common_init.clone(), eval.sgp.clone(), esumreq.clone());
                    println!("process_request, step 6");
                    if test == false {
                        return Err("Error with the other registrars".to_string());
                    }
                    Ok(res_oper)
                },
            }
        },
        GossipOperationKind::Globalgossip() => {
            println!("process_request, global gossip, step 1");
            let test = check_mkb_operation(common_init.clone(), sgp, esumreq.clone());
            println!("process_request, global gossip, step 2");
            if test == false {
                return Err("Error with the other registrars".to_string());
            }
            Ok(res_oper)
        },
    }
}

