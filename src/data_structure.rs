//use numext_fixed_hash::H256;

use types::*;
use types::SumTypeRequest::*;
use std::collections::HashMap;
use std::collections::HashSet;
use multihash::{encode};
use types::HashType;
use chrono::prelude::*;
use gossip_protocol::*;
use type_init::*;
//use std::time::Duration;
//use std::process;

//use std::io;
//use std::sync::{Arc, Mutex};
//use jsonrpc_core::{Error as JsonRpcError};


#[derive(Clone,Serialize,Deserialize)]
pub struct AccountCurrent {
    current_money: u64,
    data_current: String,
    hash: HashType,
    utc: DateTime<Utc>,
    nonce: u32
}

#[derive(Clone)]
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




pub fn func_insert_record(topic_desc: &TopicDescriptionEncode, listval: &mut Vec<AccountCurrent>, eval: AccountCurrent) -> MKBoperation {
    if topic_desc.min_interval_insertion_micros > 0 {
        let len = listval.len();
        let dura = eval.utc.signed_duration_since(listval[len-1].utc);
        let dura_micros = dura.num_microseconds();
        match dura_micros {
            Some(eval) => {
                if eval < topic_desc.min_interval_insertion_micros {
                    return MKBoperation { result: false, signature: None, text: "too near to last insertione".to_string() };
                }
            },
            None => {},
        }
    }
    listval.push(eval.clone());
    let len = listval.len();
    if topic_desc.capacity_mem > 0 || topic_desc.retention_time > 0 || topic_desc.retention_size > 0 {
        let upper_bound_retention_size = if topic_desc.retention_size > 0 {topic_desc.retention_size as usize} else {len};
        let upper_bound_retention_time = if topic_desc.retention_time > 0 {
            let mut i_level = 0;
            let dt_last = listval[len-1].utc;
            while i_level<len {
                let j_level = len - 1 - i_level;
                let dt_prev = listval[j_level].utc;
                let dura_micros = dt_last.signed_duration_since(dt_prev).num_microseconds();
                match dura_micros {
                    Some(eval) => {
                        if eval > topic_desc.retention_time { break; }
                    },
                    None => {break;},
                }
                i_level += 1;
            }
            i_level
        } else {len};
        let upper_bound_capacity_mem = if topic_desc.capacity_mem > 0 {
            let mut tot_size = 0;
            let mut i_level = 0;
            while i_level<len {
                let len = listval[len - 1 - i_level].data_current.len();
                tot_size += len;
                if tot_size > (topic_desc.capacity_mem as usize) {break;}
                i_level += 1;
            }
            i_level
        } else {len};
        let upper_bound = std::cmp::min(upper_bound_capacity_mem, std::cmp::min(upper_bound_retention_size, upper_bound_retention_time));
        if upper_bound < len {
            let nb_remove = len - upper_bound;
            for _i in 0..nb_remove {
                listval.remove(0);
            }
        }
    }
    MKBoperation { result: true, signature: Some(eval.hash), text: "success".to_string() }    
}





pub fn query_info(w: std::sync::MutexGuard<TopicAllInfo>, topic: String, name: String) -> Result<AccountCurrent, String> {
    let iter = (*w).all_topic_state.get(&topic);
    match iter {
        None => Err("Topic is not existent here".to_string()),
        Some(eval) => {
            let iter_b = eval.all_account_state.get(&name);
            match iter_b {
                None => Err("Name is not existent here".to_string()),
                Some(eval_b) => {
                    let len = eval_b.len();
                    return Ok((*eval_b)[len-1].clone());
                },
            }
        }
    }
}


pub fn compute_the_hash(topdesc: &TopicDescriptionEncode, econt: &ContainerTypeForHash) -> HashType {
    let econt_str = serde_json::to_string(econt).unwrap();
    let econt_str_u8 = econt_str.as_bytes();
    let eret = encode(topdesc.hash_method, econt_str_u8).unwrap();
    eret
}




// This function takes the request, check for correctness.
// If correct, the signature is returned to be checked.
// If not correct, then the signature is sent in order to be checked.
pub fn get_signature(mut w_mkb: std::sync::MutexGuard<TopicAllInfo>, my_reg: &SingleRegistrarFinal, eval: SumTypeRequest) -> MKBoperation {
    match eval.clone() {
        Topiccreationrequest(etop) => {
            let sgp = Default::default();
            let set_of_acct = FullTopicData { topic_desc: get_topic_desc_encode(&etop),
                                              list_active_reg: HashSet::<String>::new(),
                                              sgp: sgp,
                                              list_subscribed_node: HashSet::<String>::new(), 
                                              all_account_state: HashMap::new()};
            (*w_mkb).all_topic_state.insert(etop.topic, set_of_acct);
            MKBoperation { result: true, signature: None, text: "success".to_string() }
        },
        Accountinfo(eacc) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&eacc.topic);
            match x {
                Some(mut eacc_b) => {
                    let hash: HashType = Default::default();
                    let acct_start : AccountCurrent = AccountCurrent { current_money: 0, data_current: "".to_string(), hash: hash.clone(), utc: Utc::now(), nonce: 0};
                    eacc_b.all_account_state.insert(eacc.account_name, vec![acct_start.clone()]);
                    MKBoperation { result: true, signature: Some(acct_start.hash), text: "success".to_string() }
                },
                None => MKBoperation { result: false, signature: None, text: "topic absent".to_string() },
            }
        },
        Depositrequest(edep) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&edep.topic);
            match x {
                Some(mut edep_b) => {
                    let mut y = edep_b.all_account_state.get_mut(&edep.account_name);
                    match y {
                        Some(mut edep_c) => {
                            let len = edep_c.len();
                            if edep_c[len-1].hash == edep.hash {
                                let new_amnt = edep_c[len-1].current_money + edep.amount;
                                let econt = ContainerTypeForHash { hash: edep_c[len-1].hash.clone(), esum: eval};
                                let new_hash = compute_the_hash(&edep_b.topic_desc, &econt);
                                let new_data = "".to_string();
                                let new_nonce = edep_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), utc: Utc::now(), nonce: new_nonce};
                                func_insert_record(&edep_b.topic_desc, &mut edep_c, new_account_curr)
                            }
                            else {
                                MKBoperation { result: false, signature: None, text: "hash error".to_string() }
                            }
                        },
                        None => MKBoperation { result: false, signature: None, text: "account error".to_string() },
                    }
                },
                None => MKBoperation { result: false, signature: None, text: "topic error".to_string() },
             }
        },
        Paymentrequest(epay) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&epay.topic);
            match x {
                Some(mut epay_b) => {
                    let check_presence = |u: &FullTopicData, addr: &String| -> bool {
                        let y = u.all_account_state.get(addr);
                        match y {
                            Some(_) => true,
                            None => false,
                        }
                    };
                    let fct_corr = |u: &FullTopicData, epayreq: &PaymentRequest| -> bool {
                        if check_presence(u, &epayreq.account_name_sender) == false {
                            return false;
                        }
                        if check_presence(u, &epayreq.account_name_receiver) == false {
                            return false;
                        }
                        let stl = u.all_account_state.get(&epayreq.account_name_sender);
                        match stl {
                            Some(estl) => {
                                let len1 = estl.len();
                                if estl[len1-1].current_money < epayreq.amount {
                                    return false;
                                }
                                if estl[len1-1].hash != epayreq.hash {
                                    return false;
                                }
                                return true;
                            },
                            None => false,
                        }
                    };
                    if fct_corr(&epay_b, &epay) == false {
                        return MKBoperation { result: false, signature: None, text: "correctness error".to_string() };
                    }
                    {
                        let mut y = epay_b.all_account_state.get_mut(&epay.account_name_sender);
                        match y {
                            Some(mut esend) => {
                                let len = esend.len();
                                let new_amnt = esend[len-1].current_money - epay.amount;
                                let econt = ContainerTypeForHash { hash: esend[len-1].hash.clone(), esum: eval.clone()};
                                let new_hash1 = compute_the_hash(&epay_b.topic_desc, &econt);
                                let new_data1 = "".to_string();
                                let new_nonce = esend[len-1].nonce + 1;
                                let new_account_send = AccountCurrent { current_money: new_amnt, data_current: new_data1, hash: new_hash1.clone(), utc: Utc::now(), nonce: new_nonce};
                                let ins = func_insert_record(&epay_b.topic_desc, &mut esend, new_account_send);
                                if ins.result == false {
                                    return ins;
                                }
                            },
                            None => {},
                        }
                    }
                    {
                        let mut y = epay_b.all_account_state.get_mut(&epay.account_name_receiver);
                        match y {
                            Some(mut erecv) => {
                                let len = erecv.len();
                                let new_amnt = erecv[len-1].current_money + epay.amount;
                                let econt = ContainerTypeForHash { hash: erecv[len-1].hash.clone(), esum: eval};
                                let new_hash2 = compute_the_hash(&epay_b.topic_desc, &econt);
                                let new_data2 = "".to_string();
                                let new_nonce = erecv[len-1].nonce + 1;
                                let new_account_recv = AccountCurrent { current_money: new_amnt, data_current: new_data2, hash: new_hash2.clone(), utc: Utc::now(), nonce: new_nonce};
                                let ins = func_insert_record(&epay_b.topic_desc, &mut erecv, new_account_recv);
                                if ins.result == false {
                                    return ins;
                                }
                            },
                            None => {},
                        }
                    }
                    return MKBoperation { result: true, signature: None, text: "success".to_string() };
                },
                None => MKBoperation { result: false, signature: None, text: "topic error".to_string() },
            }
            
        },
        Withdrawrequest(ewith) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&ewith.topic);
            match x {
                Some(mut ewith_b) => {
                    let mut y = ewith_b.all_account_state.get_mut(&ewith.account_name);
                    match y {
                        Some(mut ewith_c) => {
                            let len = ewith_c.len();
                            if ewith_c[len-1].current_money > ewith.amount && ewith_c[len-1].hash == ewith.hash {
                                let new_amnt = ewith_c[len-1].current_money - ewith.amount;
                                let econt = ContainerTypeForHash { hash: ewith_c[len-1].hash.clone(), esum: eval};
                                let new_hash = compute_the_hash(&ewith_b.topic_desc, &econt);
                                let new_data = "".to_string();
                                let new_nonce = ewith_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), utc: Utc::now(), nonce: new_nonce};
                                func_insert_record(&ewith_b.topic_desc, &mut ewith_c, new_account_curr)
                            }
                            else {
                                MKBoperation { result: false, signature: None, text: "amount or hash error".to_string() }
                            }
                        },
                        None => MKBoperation { result: false, signature: None, text: "account error".to_string() },
                    }
                },
                None => MKBoperation { result: false, signature: None, text: "topic error".to_string() },
            }
        },
        Senddatarequest(edata) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&edata.topic);
            match x {
                Some(mut edata_b) => {
                    let mut y = edata_b.all_account_state.get_mut(&edata.account_name);
                    match y {
                        Some(mut edep_c) => {
                            let len = edep_c.len();
                            if edep_c[len-1].hash == edata.hash {
                                let new_amnt = edep_c[len-1].current_money;
                                let econt = ContainerTypeForHash { hash: edep_c[len-1].hash.clone(), esum: eval};
                                let new_hash = compute_the_hash(&edata_b.topic_desc, &econt);
                                let new_data = edata.data;
                                let new_nonce = edep_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), utc: Utc::now(), nonce: new_nonce};
                                func_insert_record(&edata_b.topic_desc, &mut edep_c, new_account_curr)
                            }
                            else {
                                MKBoperation { result: false, signature: None, text: "hash error".to_string() }
                            }
                        },
                        None => MKBoperation { result: false, signature: None, text: "account error".to_string() },
                    }
                },
                None => MKBoperation { result: false, signature: None, text: "topic error".to_string() },
             }
        },
        Addsubscriber(eadd) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&eadd.topic);
            match x {
                Some(mut etop_b) => {
                    let test = etop_b.list_subscribed_node.contains(&eadd.subscriber_name.clone());
                    match test {
                        true => MKBoperation { result: false, signature: None, text: "already_registered".to_string() },
                        false => {
                            etop_b.list_subscribed_node.insert(eadd.subscriber_name);
                            MKBoperation{result: true, signature: None, text: "successful insertion".to_string()}
                        },
                    }
                },
                None => MKBoperation { result: false, signature: None, text: "topic error".to_string() },
            }
            // TODO: We need a different channel for this kind of operation
            // which are 
        },
        Removesubscriber(eremove) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&eremove.topic);
            match x {
                Some(mut etop_b) => {
                    let test = etop_b.list_subscribed_node.contains(&eremove.subscriber_name);
                    match test {
                        false => MKBoperation{result: false, signature: None, text: "not_registered".to_string()},
                        true => {
                            etop_b.list_subscribed_node.remove(&eremove.subscriber_name);
                            MKBoperation{result: true, signature: None, text: "successful removal".to_string()}
                        },
                    }
                },
                None => MKBoperation { result: false, signature: None, text: "topic error".to_string() },
            }
        },
        Addregistrar(ereg) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&ereg.topic);
            match x {
                Some(mut etop_b) => {
                    let test = etop_b.list_active_reg.contains(&ereg.registrar_name.clone());
                    match test {
                        true => MKBoperation { result: false, signature: None, text: "already_registered".to_string() },
                        false => {
                            etop_b.list_active_reg.insert(ereg.registrar_name);
                            MKBoperation{result: true, signature: None, text: "successful subscriber insertion".to_string()}
                        },
                    }
                },
                None => MKBoperation { result: false, signature: None, text: "topic error".to_string() },
            }
        },
        Removeregistrar(ereg) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&ereg.topic);
            match x {
                Some(mut etop_b) => {
                    let test = etop_b.list_active_reg.contains(&ereg.registrar_name);
                    match test {
                        false => MKBoperation{result: false, signature: None, text: "not_registered".to_string()},
                        true => {
                            etop_b.list_active_reg.remove(&ereg.registrar_name);
                            MKBoperation{result: true, signature: None, text: "successful registrar removal".to_string()}
                        },
                    }
                },
                None => MKBoperation { result: false, signature: None, text: "topic error".to_string() },
            }
        },
        
    }
}
