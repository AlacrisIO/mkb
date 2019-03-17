use secp256k1::{Secp256k1, Message};
use multihash::encode;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedString {
    pub result: String,
    pub sig: Vec<u8>,
}

pub fn signature_oper_secp256k1(secret_key: secp256k1::key::SecretKey, estr: &String) -> SignedString {
    let estr_u8 : &[u8] = estr.as_bytes();
    let estr_u8_b = get_vector_len_thirtytwo(estr_u8);
    let estr_u8_b_ref : &[u8] = &estr_u8_b;
    let secp = Secp256k1::new();
    let message = Message::from_slice(estr_u8_b_ref).expect("signature_oper_secp256k1 : Error in creation of message");
    let sig : secp256k1::Signature = secp.sign(&message, &secret_key);
    let sig_vec : Vec<u8> = secp256k1::Signature::serialize_der(&sig);
    SignedString {result: estr.to_string(), sig: sig_vec}
}



pub fn get_vector_len_thirtytwo(v: &[u8]) -> Vec<u8> {
    let e_vec = encode(multihash::Hash::Keccak256, v).expect("encoding failed");
    let mut e_vec_ret = Vec::<u8>::new();
    for i in 0..32 {
        e_vec_ret.push(e_vec[i]);
    }
    e_vec_ret
}
