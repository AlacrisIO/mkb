use types::*;
use type_sign::*;
use types::SumTypeAnswer::*;
//use types::SumTypeRequest::*;
use type_init::*;
use std::cmp::{Ordering, min};
use multihash::encode;
use gossip_protocol::send_transaction;

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct RetrieveHashForVRF {
    pub topic: String,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AnswerHashForVRF {
    pub hash: String,
}


#[derive(Debug, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct PairAddrHash {
    pub addr: String,
    pub hash: String,
}

impl PartialOrd for PairAddrHash {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PairAddrHash {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash.cmp(&other.hash)
    }
}

impl PartialEq for PairAddrHash {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}




#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Committee {
    pub topic: String,
    pub committee: Vec<String>,
    pub list_pair: Vec<PairAddrHash>,
}




pub fn compute_vrf_hash(eval: &FullTopicData, topic: String) -> AnswerHashForVRF {
    let mut ev = Vec::<SinglePairUserHash>::new();
    for (name, seq) in &eval.all_account_state {
        let elen = seq.len();
        let esing = SinglePairUserHash { account_name: name.to_string(), hash: seq[elen-1].hash.clone() };
        ev.push(esing);
    }
    let econt = ComputeHashOfTopic { topic: topic, topic_desc: eval.topic_desc.clone(), list_pair: ev };
    let econt_str = serde_json::to_string(&econt).expect("error in serialization");
    let econt_str_u8 = econt_str.as_bytes();
    let eret = encode(eval.topic_desc.hash_method.val, econt_str_u8).unwrap();
    let eret_str = bytes_to_hex(eret);
    let ans = AnswerHashForVRF { hash: eret_str};
    ans
}


pub fn compute_committee_and_send(x_ftd: &mut FullTopicData, my_reg: &SingleRegistrarFinal, topic: String, committee_size: i32) -> bool {
    println!("compute_committee_and_send, step 1");
    let mut vec_pair = Vec::<PairAddrHash>::new();
    //
    // inserting the hash of the current node
    //
    let evrf_hash = compute_vrf_hash(x_ftd, topic.clone());
    vec_pair.push(PairAddrHash{addr: my_reg.address.clone(), hash: evrf_hash.hash});
    //
    // iterating over the neighbors
    //
    let eval = RetrieveHashForVRF { topic: topic.clone()};
    let esumreq = SumTypeRequest::Retrievehashforvrf(eval);
    for e_reg in x_ftd.sgp.list_neighbor.clone() {
        let reply = send_transaction(&e_reg, &esumreq);
        match reply {
            None => {
                return false;
            },
            Some(x) => {
                if x.result {
                    match x.answer {
                        Answerhashforvrf(ehash) => {
                            vec_pair.push(PairAddrHash{addr: e_reg.address, hash: ehash.hash});
                        },
                        _ => {},
                    }
                }
                else {
                    return false;
                }
            },
        }
    }
    println!("compute_committee_and_send, step 2");
    //
    // Computing the committee
    //
    vec_pair.sort();
    let mut e_vec = Vec::<String>::new();
    let len = min(vec_pair.len(), committee_size as usize);
    for i in 0..len {
        e_vec.push(vec_pair[i].addr.clone());
    }
    let ecomm = Committee { topic: topic, committee: e_vec.clone(), list_pair: vec_pair};
    println!("compute_committee_and_send, step 3");
    //
    // Sending the committee
    //
    let esumreq_comm = SumTypeRequest::Setcommittee(ecomm);
    for e_reg in x_ftd.sgp.list_neighbor.clone() {
        let reply = send_transaction(&e_reg, &esumreq_comm);
        match reply {
            None => {
                return false;
            },
            Some(x) => {
                if x.result==false {
                    return false;
                }
            },
        }
    }
    println!("compute_committee_and_send, step 4");
    //
    // Setting the committee
    //
    x_ftd.committee = e_vec;
    true
}
