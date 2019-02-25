use merkle_cbt;
use merkle_cbt::Merge;
use merkle_cbt::MerkleTree;
use numext_fixed_hash::H256;

use types::*;
use std::io;
use std::collections::HashMap;
//use std::sync::{Arc, Mutex};
use jsonrpc_core::{Error as JsonRpcError};



#[derive(Default,Serialize,Deserialize)]
pub struct AccountCurrent {
    current_money: u64,
    data_current: String,
    the_hash: H256
}

#[derive(Default,Serialize,Deserialize)]
pub struct SetOfAccount {
    pub all_account_state: HashMap<String,Vec<AccountCurrent>>
}

#[derive(Default,Serialize,Deserialize)]
pub struct TopicAllInfo {
    pub all_topic_state: HashMap<String,SetOfAccount>
}


pub fn query_info(mut w: std::sync::MutexGuard<TopicAllInfo>, topic: String, name: String) -> Result<AccountCurrent, String> {
    let iter = (*w).all_topic_state.get(&topic);
    match iter {
        None => Err("Topic is not existent here".to_string()),
        Some(eval) => {
            let iter_b = eval.all_account_state.get(&name);
            match iter_b {
                None => Err("Name is not existent here".to_string()),
                Some(eval_b) => {
                    let len = (*eval_b).len();
                    return Ok((*eval_b)[len-1]);
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
pub fn get_signature(tot_mkb: TopicAllInfo, eval: SumTypeRequest) -> MerkleVerification {
    


    
    let merkl = MerkleVerification { result: true, signature: None };
    merkl
}
