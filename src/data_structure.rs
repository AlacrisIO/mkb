//use merkle_cbt;
//use merkle_cbt::Merge;
//use merkle_cbt::MerkleTree;
use numext_fixed_hash::H256;

use types::*;
use types::SumTypeRequest::*;
//use std::io;
use std::collections::HashMap;
//use std::sync::{Arc, Mutex};
//use jsonrpc_core::{Error as JsonRpcError};



#[derive(Clone,Default,Serialize,Deserialize)]
pub struct AccountCurrent {
    current_money: u64,
    data_current: String,
    hash: H256
}

#[derive(Clone,Default,Serialize,Deserialize)]
pub struct SetOfAccount {
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


/*
#[derive(Default)]
pub struct AllDataMerkleTree {
    pub 
    pub account_map : MerkleTree<H256>,
    pub token_map : MerkleTree<H256>,
    pub transaction_map : MerkleTree<H256>,
}
*/







// This function takes the request, check for correctness.
// If correct, the signature is returned to be checked.
// If not correct, then the signature is sent in order to be checked.
pub fn get_signature(mut w_mkb: std::sync::MutexGuard<TopicAllInfo>, eval: SumTypeRequest) -> MerkleVerification {
    match eval {
        Topiccreationrequest(etop) => {
            let set_of_acct: SetOfAccount = Default::default();
            (*w_mkb).all_topic_state.insert(etop.topic, set_of_acct);
            MerkleVerification { result: true, signature: None }
        },
        Accountinfo(eacc) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&eacc.topic);
            match x {
                Some(mut eacc_b) => {
                    let acct_start : AccountCurrent = Default::default();
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
                            let new_amnt = edep_c[len-1].current_money + edep.amount;
                            let new_hash = edep_c[len-1].hash.clone(); // Obviously wrong
                            let new_data = "".to_string();
                            let new_account_curr = AccountCurrent { current_money: new_amnt, data_current: new_data, hash: new_hash.clone()};
                            edep_c.push(new_account_curr);
                            MerkleVerification { result: false, signature: Some(new_hash) }
                        },
                        None => MerkleVerification { result: false, signature: None },
                    }
                },
                None => MerkleVerification { result: false, signature: None },
             }
        },

/*        
        Paymentrequest(epay) => {
            let mut x = (*w_mkb).all_topic_state.get_mut(&epay.topic);
            match x {
                Some(mut epay_b) => {
                    let mut y = epay_b.all_account_state.get_mut(&epay.account_name_sender);
                    match y {
                        Some(mut esend) => {
                            let mut z = epay_b.all_account_state.get_mut(&epay.account_name_receiver);
                            match z {
                                Some(mut erecv) => {
                                    let len1 = esend.len();
                                    if esend[len1-1].current_money < epay.amount {
                                        return MerkleVerification { result: false, signature: None };
                                    }
                                    else {
                                        let new_amnt = esend[len1-1].current_money - epay.amount;
                                        let new_hash1 = esend[len1-1].hash.clone(); // Obviously wrong
                                        let new_data1 = "".to_string();
                                        let new_account_send = AccountCurrent { current_money: new_amnt, data_current: new_data1, hash: new_hash1.clone()};
                                        esend.push(new_account_send);
                                        //
                                        let len2 = erecv.len();
                                        let new_amnt = erecv[len2-1].current_money + epay.amount;
                                        let new_hash2 = erecv[len2-1].hash.clone(); // Obviously wrong
                                        let new_data2 = "".to_string();
                                        let new_account_send = AccountCurrent { current_money: new_amnt, data_current: new_data2, hash: new_hash2.clone()};
                                        erecv.push(new_account_send);
                                        MerkleVerification { result: true, signature: None }
                                    }
                                },
                                None => MerkleVerification { result: false, signature: None },
                            }
                        }
                        None => MerkleVerification { result: false, signature: None },
                    }
                },
                None => MerkleVerification { result: false, signature: None },
            }
            
        },
*/
        _ => MerkleVerification { result: true, signature: None },
    }
}
