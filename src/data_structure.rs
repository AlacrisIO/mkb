//use numext_fixed_hash::H256;
//use std::time::Duration;
//use std::process;
//use std::io;
//use std::sync::{Arc, Mutex};

use types::*;
use types::SumTypeRequest::*;
use gossip_protocol::*;
use std::collections::{BTreeSet, BTreeMap};
use multihash::{encode};
use chrono::prelude::*;
use types::HashType;
use type_init::*;
use vrf::compute_vrf_hash;
use type_sign::vecu8_to_string;


pub fn func_insert_record(topic_desc: &TopicDescription, listval: &mut Vec<AccountCurrent>, eval: AccountCurrent) -> TypeAnswer {
    /*
    if topic_desc.min_interval_insertion_micros > 0 {
        let len = listval.len();
        let dura = eval.utc.signed_duration_since(listval[len-1].utc);
        let dura_micros = dura.num_microseconds();
        match dura_micros {
            Some(eval) => {
                if eval < topic_desc.min_interval_insertion_micros {
                    let mkb_oper_triv = SumTypeAnswer::Mkboperation(MKBoperation {hash: None});
                    return TypeAnswer { result: false, answer: mkb_oper_triv, text: "too near to last insertione".to_string() };
                }
            },
            None => {},
        }
    }*/
    listval.push(eval.clone());
    let len = listval.len();
    if topic_desc.total_capacity_mem > 0 || topic_desc.retention_time > 0 || topic_desc.retention_size > 0 {
        let upper_bound_retention_size = if topic_desc.retention_size > 0 {topic_desc.retention_size as usize} else {len};
        let upper_bound_retention_time = len;
/*
        let upper_bound_retention_time = if topic_desc.retention_time > 0 {
            let mut i_level = 0;
            let dt_last = listval[len-1].utc;
            while i_level<len {
                let j_level = len - 1 - i_level;
                let dt_prev = listval[j_level].utc;
                let dura_micros = dt_last.signed_duration_since(dt_prev).num_microseconds();
                match dura_micros {
                    Some(eper_time) => {
                        if eper_time > topic_desc.retention_time { break; }
                    },
                    None => {break;},
                }
s                i_level += 1;
            }
            i_level
        } else {len}; */
        let upper_bound_capacity_mem = if topic_desc.total_capacity_mem > 0 {
            let mut tot_size = 0;
            let mut i_level = 0;
            while i_level<len {
                let len = listval[len - 1 - i_level].data_current.len();
                tot_size += len;
                if tot_size > (topic_desc.total_capacity_mem as usize) {break;}
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
    let mkb_oper = SumTypeAnswer::Mkboperation(MKBoperation {hash: Some(vecu8_to_string(eval.hash))});
    TypeAnswer { result: true, answer: mkb_oper, text: "success".to_string() }
}





pub fn query_info_latest(w_mkb: &std::sync::MutexGuard<TopicAllInfo>, topic: String, name: String) -> Result<AccountCurrent, String> {
    let iter = (*w_mkb).all_topic_state.get(&topic);
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


pub fn triple_query_info(w_mkb: &std::sync::MutexGuard<TopicAllInfo>, topic: String, name: String, nonce: u32) -> Result<AccountCurrent, String> {
    let iter = (*w_mkb).all_topic_state.get(&topic);
    match iter {
        None => Err("Topic is not existent on this registrar".to_string()),
        Some(eval) => {
            let iter_b = eval.all_account_state.get(&name);
            match iter_b {
                None => Err("Name is not existent here".to_string()),
                Some(eval_b) => {
                    for eent in eval_b {
                        if eent.nonce==nonce {
                            return Ok(eent.clone())
                        }
                    }
                    Err("nonce is absent in the list".to_string())
                },
            }
        }
    }
}




pub fn compute_the_hash(topdesc: &TopicDescription, econt: &ContainerTypeForHash) -> HashType {
    let econt_str = serde_json::to_string(econt).expect("Error in compute_the_hash");
    let econt_str_u8 = econt_str.as_bytes();
    let eret = encode(topdesc.hash_method.val, econt_str_u8).expect("encoding failed");
    let eret_red = eret[0..32].to_vec();
    println!("compute_the_hash |eret_red|={:?}", eret_red.len());
    eret_red
}





pub fn get_topic_info_wmkb(w_mkb: &std::sync::MutexGuard<TopicAllInfo>, my_reg: &SingleRegistrarFinal, topic: &String) -> Option<ExportTopicInformation> {
    let x = (*w_mkb).all_topic_state.get(topic);
    match x {
        Some(eval) => {
            let mut e_vec = Vec::<String>::new();
            for e_ent in eval.list_active_reg.clone() {
                e_vec.push(e_ent);
            }
            Some(ExportTopicInformation {topic_desc: eval.topic_desc.clone(),
                one_registrar_ip_addr: my_reg.ip_addr.clone(),
                one_registrar_port: my_reg.port,
                list_registrar_name: e_vec})
        },
        None => None,
    }
}


pub fn has_topic(w_mkb: &mut std::sync::MutexGuard<TopicAllInfo>, etopic: &String) -> bool {
    let x = (*w_mkb).all_topic_state.get(etopic);
    match x {
        Some(_) => {true},
        None => {false},
    }
}

pub fn has_account(eacc : &mut FullTopicData, eacc_name: &String) -> bool {
    let x = eacc.all_account_state.get(eacc_name);
    match x {
        Some(_) => {true},
        None => {false},
    }
}


// This function takes the request, check for correctness.
// If correct, the signature is returned to be checked.
// If not correct, then the signature is sent in order to be checked.
pub fn process_operation(w_mkb: &mut std::sync::MutexGuard<TopicAllInfo>, common_init: CommonInitFinal, my_reg: &SingleRegistrarFinal, esumreq: SumTypeRequest) -> TypeAnswer {
    let triv_answer = SumTypeAnswer::Trivialanswer(TrivialAnswer {});
    match esumreq.clone() {
        Topiccreationrequest(etop) => {
            //
            let test = has_topic(w_mkb, &etop.topic);
            match test {
                true => {
                    let x = (*w_mkb).all_topic_state.get(&etop.topic);
                    match x {
                        Some(e_rec) => {
                            if etop == e_rec.topic_desc {
                                return TypeAnswer { result: true, answer: triv_answer, text: "topic already existing and the same topic description".to_string() };
                            }
                            return TypeAnswer { result: false, answer: triv_answer, text: "The topic already exist but the topic description put is different from the one already existing".to_string() };
                        },
                        None => {
                            return TypeAnswer { result: false, answer: triv_answer, text: "bug in the code".to_string() };
                        },
                    }
                },
                false => {
                    let sgp = Default::default(); // for just one node, the trivial sgp is ok.
                    let mut e_list = BTreeSet::<String>::new();
                    e_list.insert(my_reg.address.clone());
                    let set_of_acct = FullTopicData { topic_desc: etop.clone(),
                                                      list_active_reg: e_list,
                                                      sgp: sgp,
                                                      committee: Vec::<String>::new(),
                                                      list_subscribed_node: BTreeSet::<String>::new(),
                                                      all_account_state: BTreeMap::new()};
                    (*w_mkb).all_topic_state.insert(etop.topic, set_of_acct);
                    TypeAnswer { result: true, answer: triv_answer, text: "success".to_string() }
                },
            }
        },
        Accountinfo(eacc) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&eacc.topic);
            match x {
                Some(mut eacc_b) => {
                    let mut hash: Vec<u8> = Vec::new();
                    for _i in 0..32 {
                        hash.push(0);
                    }
                    let acct_start : AccountCurrent = AccountCurrent { current_money: 0, data_current: "".to_string(), hash: hash.clone(), name: eacc.clone().account_name, nonce: 0};
                    let mkb_oper = SumTypeAnswer::Mkboperation(MKBoperation {hash: Some(vecu8_to_string(acct_start.clone().hash))});
                    let test = has_account(eacc_b, &eacc.clone().account_name);
                    match test {
                        true => {
                            TypeAnswer { result: true, answer: mkb_oper, text: "success".to_string() }
                        },
                        false => {
                            eacc_b.all_account_state.insert(eacc.account_name, vec![acct_start.clone()]);
                            TypeAnswer { result: true, answer: mkb_oper, text: "success".to_string() }
                        }
                    }
                },
                None => TypeAnswer { result: false, answer: triv_answer, text: "topic absent".to_string() },
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
                                let econt = ContainerTypeForHash { hash: edep_c[len-1].hash.clone(), esum: esumreq};
                                let new_hash = compute_the_hash(&edep_b.topic_desc, &econt);
                                let new_data = "".to_string();
                                let new_nonce = edep_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), name: edep.account_name, nonce: new_nonce};
                                func_insert_record(&edep_b.topic_desc, &mut edep_c, new_account_curr)
                            }
                            else {
                                TypeAnswer { result: false, answer: triv_answer, text: "hash error".to_string() }
                            }
                        },
                        None => TypeAnswer { result: false, answer: triv_answer, text: "account error".to_string() },
                    }
                },
                None => TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() },
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
                        return TypeAnswer { result: false, answer: triv_answer, text: "correctness error".to_string() };
                    }
                    {
                        let mut y = epay_b.all_account_state.get_mut(&epay.account_name_sender);
                        match y {
                            Some(mut esend) => {
                                let len = esend.len();
                                let new_amnt = esend[len-1].current_money - epay.amount;
                                let econt = ContainerTypeForHash { hash: esend[len-1].hash.clone(), esum: esumreq.clone()};
                                let new_hash1 = compute_the_hash(&epay_b.topic_desc, &econt);
                                let new_data1 = "".to_string();
                                let new_nonce = esend[len-1].nonce + 1;
                                let new_account_send = AccountCurrent { current_money: new_amnt, data_current: new_data1, hash: new_hash1.clone(), name: epay.account_name_sender, nonce: new_nonce};
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
                                let econt = ContainerTypeForHash { hash: erecv[len-1].hash.clone(), esum: esumreq};
                                let new_hash2 = compute_the_hash(&epay_b.topic_desc, &econt);
                                let new_data2 = "".to_string();
                                let new_nonce = erecv[len-1].nonce + 1;
                                let new_account_recv = AccountCurrent { current_money: new_amnt, data_current: new_data2, hash: new_hash2.clone(), name: epay.account_name_receiver, nonce: new_nonce};
                                let ins = func_insert_record(&epay_b.topic_desc, &mut erecv, new_account_recv);
                                if ins.result == false {
                                    return ins;
                                }
                            },
                            None => {},
                        }
                    }
                    return TypeAnswer { result: true, answer: triv_answer, text: "success".to_string() };
                },
                None => TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() },
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
                                let econt = ContainerTypeForHash { hash: ewith_c[len-1].hash.clone(), esum: esumreq};
                                let new_hash = compute_the_hash(&ewith_b.topic_desc, &econt);
                                let new_data = "".to_string();
                                let new_nonce = ewith_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), name: ewith.account_name, nonce: new_nonce};
                                func_insert_record(&ewith_b.topic_desc, &mut ewith_c, new_account_curr)
                            }
                            else {
                                TypeAnswer { result: false, answer: triv_answer, text: "amount or hash error".to_string() }
                            }
                        },
                        None => TypeAnswer { result: false, answer: triv_answer, text: "account error".to_string() },
                    }
                },
                None => TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() },
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
                            //  println!("edep_c[len-1]={:?}", edep_c[len-1].hash);
                            //  println!("data.hash={:?}", edata.hash);
                            // First case, straightforward insertion
                            if edep_c[len-1].hash == edata.hash {
                                let new_amnt = edep_c[len-1].current_money;
                                let econt = ContainerTypeForHash { hash: edep_c[len-1].hash.clone(), esum: esumreq};
                                let new_hash = compute_the_hash(&edata_b.topic_desc, &econt);
                                let new_data = edata.data;
                                let new_nonce = edep_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), name: edata.account_name, nonce: new_nonce};
                                return func_insert_record(&edata_b.topic_desc, &mut edep_c, new_account_curr);
                            }
                            // Second case, idempotent operation
                            for i in 0..len-1 {
                                if edep_c[i].hash == edata.hash {
                                    let econt = ContainerTypeForHash { hash: edep_c[i].hash.clone(), esum: esumreq};
                                    let new_hash = compute_the_hash(&edata_b.topic_desc, &econt);
                                    if new_hash != edep_c[i+1].hash {
                                        return TypeAnswer { result: false, answer: triv_answer, text: "the hash is an old one but the entry is not coherent with that previous history".to_string() };
                                    }
                                    let mkb_oper = SumTypeAnswer::Mkboperation(MKBoperation {hash: Some(vecu8_to_string(edep_c[i+1].clone().hash))});
                                    return TypeAnswer { result: true, answer: mkb_oper, text: "entry is already present but the operation is coherent, it just does nothing".to_string() };
                                }
                            }
                            TypeAnswer { result: false, answer: triv_answer, text: "hash error".to_string() }
                        },
                        None => TypeAnswer { result: false, answer: triv_answer, text: "account error".to_string() },
                    }
                },
                None => TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() },
             }
        },
        Addsubscriber(eadd) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&eadd.topic);
            match x {
                Some(mut etop_b) => {
                    let test = etop_b.list_subscribed_node.contains(&eadd.subscriber_name.clone());
                    match test {
                        true => TypeAnswer { result: false, answer: triv_answer, text: "already_registered".to_string() },
                        false => {
                            etop_b.list_subscribed_node.insert(eadd.subscriber_name);
                            TypeAnswer{result: true, answer: triv_answer, text: "successful insertion".to_string()}
                        },
                    }
                },
                None => TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() },
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
                        false => TypeAnswer{result: false, answer: triv_answer, text: "not_registered".to_string()},
                        true => {
                            etop_b.list_subscribed_node.remove(&eremove.subscriber_name);
                            TypeAnswer{result: true, answer: triv_answer, text: "successful removal".to_string()}
                        },
                    }
                },
                None => TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() },
            }
        },
        Addregistrar(ereg) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&ereg.topic);
            match x {
                Some(mut etop_b) => {
                    let test = etop_b.list_active_reg.contains(&ereg.registrar_address.clone());
                    match test {
                        true => TypeAnswer { result: false, answer: triv_answer, text: "already_registered".to_string() },
                        false => {
                            etop_b.list_active_reg.insert(ereg.registrar_address);
                            etop_b.sgp = compute_simple_gossip_protocol_topic(&common_init, my_reg.address.clone(), etop_b.list_active_reg.clone());
                            TypeAnswer{result: true, answer: triv_answer, text: "successful subscriber insertion".to_string()}
                        },
                    }
                },
                None => TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() },
            }
        },
        Removeregistrar(ereg) => {
            if ereg.registrar_address == my_reg.address {
                let x = (*w_mkb).all_topic_state.remove(&ereg.topic);
                match x {
                    Some(_) => TypeAnswer{result: false, answer: triv_answer, text: "error in registrar removal".to_string()},
                    None => TypeAnswer{result: true, answer: triv_answer, text: "successful registrar removal".to_string()}
                }
            }
            else {
                let mut x = (*w_mkb).all_topic_state.get_mut(&ereg.topic);
                match x {
                    Some(mut etop_b) => {
                        let test = etop_b.list_active_reg.contains(&ereg.registrar_address);
                        match test {
                            false => TypeAnswer{result: false, answer: triv_answer, text: "not_registered".to_string()},
                            true => {
                                etop_b.list_active_reg.remove(&ereg.registrar_address);
                                etop_b.sgp = compute_simple_gossip_protocol_topic(&common_init, my_reg.address.clone(), etop_b.list_active_reg.clone());
                                TypeAnswer{result: true, answer: triv_answer, text: "successful registrar removal".to_string()}
                            },
                        }
                    },
                    None => TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() },
                }
            }
        },
        Internalrequesttopicinfo(eint) => {
            let eval = get_topic_info_wmkb(w_mkb, my_reg, &eint.topic);
            match eval {
                None => {
                    TypeAnswer { result: false, answer: triv_answer, text: "topic is absent".to_string() }
                },
                Some(eval) => {
                    let export_topic_succ = SumTypeAnswer::Exporttopicinformation(eval);
                    TypeAnswer { result: true, answer: export_topic_succ, text: "success".to_string() }
                },
            }
        },
        Fulltopicexport(etopicexport) => {
            (*w_mkb).all_topic_state.insert(etopicexport.topic, etopicexport.topic_info);
            let triv_ans = SumTypeAnswer::Trivialanswer(TrivialAnswer{});
            TypeAnswer { result: true, answer: triv_ans, text: "success".to_string() }
        },
        Retrievehashforvrf(eret) => {
            let x = (*w_mkb).all_topic_state.get(&eret.topic);
            match x {
                None => {
                    TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() }
                },
                Some(eval) => {
                    let hash_vrf = compute_vrf_hash(eval, eret.topic);
                    let ans = SumTypeAnswer::Answerhashforvrf(hash_vrf);
                    TypeAnswer { result: true, answer: ans, text: "successful topic extraction".to_string() }
                },
            }
        },
        Setcommittee(ecomm) => {
            let x = (*w_mkb).all_topic_state.get_mut(&ecomm.topic);
            match x {
                None => {
                    TypeAnswer { result: false, answer: triv_answer, text: "topic error".to_string() }
                },
                Some(eval) => {
                    eval.committee = ecomm.committee;
                    TypeAnswer { result: true, answer: triv_answer, text: "successful committee setting".to_string() }
                },
            }
        },
        Getlatestentry(ereq) => {
            let e_ans = query_info_latest(w_mkb, ereq.topic, ereq.account_name);
            match e_ans {
                Err(eval) => {
                    TypeAnswer { result: false, answer: triv_answer, text: eval }
                },
                Ok(eval) => {
                    let ans = SumTypeAnswer::Accountlatestrequest(eval);
                    TypeAnswer { result: true, answer: ans, text: "successful request".to_string() }
                },
            }
        },
        Triplerequest(ereq) => {
            let e_ans = triple_query_info(w_mkb, ereq.topic, ereq.account_name, ereq.nonce);
            match e_ans {
                Err(eval) => {
                    TypeAnswer { result: false, answer: triv_answer, text: eval }
                },
                Ok(eval) => {
                    let ans = SumTypeAnswer::Accounttriplerequest(eval);
                    TypeAnswer { result: true, answer: ans, text: "successful request".to_string() }
                },
            }
        },
//        _ => {TypeAnswer { result: false, answer: triv_answer, text: "successful request".to_string() }},
    }
}
