//use numext_fixed_hash::H256;

use types::*;
use types::SumTypeRequest::*;
use std::collections::HashMap;
use multihash::{encode, Hash};
use types::HashType;
use chrono::prelude::*;
//use std::process;
//use merkle_cbt;
//use merkle_cbt::Merge;
//use merkle_cbt::MerkleTree;

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

#[derive(Clone,Default,Serialize,Deserialize)]
pub struct SetOfAccount {
    pub topic_desc: TopicDescription,
    pub all_account_state: HashMap<String,Vec<AccountCurrent>>
}

#[derive(Clone,Default,Serialize,Deserialize)]
pub struct TopicAllInfo {
    pub all_topic_state: HashMap<String,SetOfAccount>
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


pub fn compute_the_hash(econt: &ContainerTypeForHash) -> HashType {
    let econt_str = serde_json::to_string(econt).unwrap();
    let econt_str_u8 = econt_str.as_bytes();
    let eret = encode(Hash::SHA3256, econt_str_u8).unwrap();
    eret
}




// This function takes the request, check for correctness.
// If correct, the signature is returned to be checked.
// If not correct, then the signature is sent in order to be checked.
pub fn get_signature(mut w_mkb: std::sync::MutexGuard<TopicAllInfo>, eval: SumTypeRequest) -> MerkleVerification {
    match eval.clone() {
        Topiccreationrequest(etop) => {
            let set_of_acct: SetOfAccount = Default::default();
            (*w_mkb).all_topic_state.insert(etop.topic, set_of_acct);
            MerkleVerification { result: true, signature: None }
        },
        Accountinfo(eacc) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&eacc.topic);
            match x {
                Some(mut eacc_b) => {
                    let hash: HashType = Default::default();
                    let acct_start : AccountCurrent = AccountCurrent { current_money: 0, data_current: "".to_string(), hash: hash.clone(), utc: Utc::now(), nonce: 0};
                    eacc_b.all_account_state.insert(eacc.account_name, vec![acct_start.clone()]);
                    MerkleVerification { result: false, signature: Some(acct_start.hash) }
                },
                None => MerkleVerification { result: false, signature: None },
            }
        },
        Depositrequest(edep) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&edep.topic);
            match x {
                Some(mut edep_b) => {
                    let mut y = edep_b.all_account_state.get_mut(&edep.account_name);
                    match y {
                        Some(edep_c) => {
                            let len = edep_c.len();
                            if edep_c[len-1].hash == edep.hash {
                                let new_amnt = edep_c[len-1].current_money + edep.amount;
                                let econt = ContainerTypeForHash { hash: edep_c[len-1].hash.clone(), esum: eval};
                                let new_hash = compute_the_hash(&econt);
                                let new_data = "".to_string();
                                let new_nonce = edep_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), utc: Utc::now(), nonce: new_nonce};
                                edep_c.push(new_account_curr);
                                MerkleVerification { result: true, signature: Some(new_hash) }
                            }
                            else {
                                MerkleVerification { result: false, signature: None }
                            }
                        },
                        None => MerkleVerification { result: false, signature: None },
                    }
                },
                None => MerkleVerification { result: false, signature: None },
             }
        },
        Paymentrequest(epay) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&epay.topic);
            match x {
                Some(mut epay_b) => {
                    let check_presence = |u: &SetOfAccount, addr: &String| -> bool {
                        let y = u.all_account_state.get(addr);
                        match y {
                            Some(_) => true,
                            None => false,
                        }
                    };
                    let fct_corr = |u: &SetOfAccount, epayreq: &PaymentRequest| -> bool {
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
                        return MerkleVerification { result: false, signature: None };
                    }
                    {
                        let mut y = epay_b.all_account_state.get_mut(&epay.account_name_sender);
                        match y {
                            Some(esend) => {
                                let len = esend.len();
                                let new_amnt = esend[len-1].current_money - epay.amount;
                                let econt = ContainerTypeForHash { hash: esend[len-1].hash.clone(), esum: eval.clone()};
                                let new_hash1 = compute_the_hash(&econt);
                                let new_data1 = "".to_string();
                                let new_nonce = esend[len-1].nonce + 1;
                                let new_account_send = AccountCurrent { current_money: new_amnt, data_current: new_data1, hash: new_hash1.clone(), utc: Utc::now(), nonce: new_nonce};
                                esend.push(new_account_send);
                            },
                            None => {},
                        }
                    }
                    {
                        let mut y = epay_b.all_account_state.get_mut(&epay.account_name_receiver);
                        match y {
                            Some(erecv) => {
                                let len = erecv.len();
                                let new_amnt = erecv[len-1].current_money + epay.amount;
                                let econt = ContainerTypeForHash { hash: erecv[len-1].hash.clone(), esum: eval};
                                let new_hash2 = compute_the_hash(&econt);
                                let new_data2 = "".to_string();
                                let new_nonce = erecv[len-1].nonce + 1;
                                let new_account_send = AccountCurrent { current_money: new_amnt, data_current: new_data2, hash: new_hash2.clone(), utc: Utc::now(), nonce: new_nonce};
                                erecv.push(new_account_send);
                            },
                            None => {},
                        }
                    }
                    return MerkleVerification { result: true, signature: None };
                },
                None => MerkleVerification { result: false, signature: None },
            }
            
        },
        Withdrawrequest(ewith) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&ewith.topic);
            match x {
                Some(mut ewith_b) => {
                    let mut y = ewith_b.all_account_state.get_mut(&ewith.account_name);
                    match y {
                        Some(ewith_c) => {
                            let len = ewith_c.len();
                            if ewith_c[len-1].current_money > ewith.amount && ewith_c[len-1].hash == ewith.hash {
                                let new_amnt = ewith_c[len-1].current_money - ewith.amount;
                                let econt = ContainerTypeForHash { hash: ewith_c[len-1].hash.clone(), esum: eval};
                                let new_hash = compute_the_hash(&econt);
                                let new_data = "".to_string();
                                let new_nonce = ewith_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), utc: Utc::now(), nonce: new_nonce};
                                ewith_c.push(new_account_curr);
                                MerkleVerification { result: true, signature: Some(new_hash) }
                            }
                            else {
                                MerkleVerification { result: false, signature: None }
                            }
                        },
                        None => MerkleVerification { result: false, signature: None },
                    }
                },
                None => MerkleVerification { result: false, signature: None },
            }
        },
        Senddatarequest(edata) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&edata.topic);
            match x {
                Some(mut edata_b) => {
                    let mut y = edata_b.all_account_state.get_mut(&edata.account_name);
                    match y {
                        Some(edep_c) => {
                            let len = edep_c.len();
                            if edep_c[len-1].hash == edata.hash {
                                let new_amnt = edep_c[len-1].current_money;
                                let econt = ContainerTypeForHash { hash: edep_c[len-1].hash.clone(), esum: eval};
                                let new_hash = compute_the_hash(&econt);
                                let new_data = edata.data;
                                let new_nonce = edep_c[len-1].nonce + 1;
                                let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone(), utc: Utc::now(), nonce: new_nonce};
                                edep_c.push(new_account_curr);
                                MerkleVerification { result: true, signature: Some(new_hash) }
                            }
                            else {
                                MerkleVerification { result: false, signature: None }
                            }
                        },
                        None => MerkleVerification { result: false, signature: None },
                    }
                },
                None => MerkleVerification { result: false, signature: None },
             }
        },
    }
}
