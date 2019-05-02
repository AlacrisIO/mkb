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


pub fn check_signature_oper(public_key: secp256k1::key::PublicKey, str_sig: &SignedString) -> bool {
    let estr_u8 : &[u8] = str_sig.result.as_bytes();
    let estr_u8_b = get_vector_len_thirtytwo(estr_u8);
    let message = Message::from_slice(&estr_u8_b).expect("send_transaction : Error in creation of message");
    let secp = Secp256k1::new();
    let esign : secp256k1::Signature = secp256k1::Signature::from_der(&str_sig.sig).expect("send_transaction : Error in extraction of signature");
    let test : bool = secp.verify(&message, &esign, &public_key).is_ok();
    test
}




pub fn get_vector_len_thirtytwo(v: &[u8]) -> Vec<u8> {
    let e_vec = encode(multihash::Hash::Keccak256, v).expect("encoding failed");
    let mut e_vec_ret = Vec::<u8>::new();
    for i in 0..32 {
        e_vec_ret.push(e_vec[i]);
    }
    e_vec_ret
}


pub fn position_in_vector(v: &Vec<&str>, val: &str) -> usize {
    let len = v.len();
    for i in 0..len {
        if v[i] == val {
            return i;
        }
    }
    len
}

pub fn string_to_vecu8(vin: String) -> Vec<u8> {
    let len = vin.len();
    if len == 0 {
        let mut e_vec = Vec::new();
        for i in 0..32 {
            e_vec.push(0);
        }
        return e_vec;
    }
    let len2=len/2;
    let vec_char : Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
    let mut e_vec = Vec::new();
    for i in 0..len2 {
        let char1 : &str = &vin[2*i..2*i+1];
        let char2 : &str = &vin[2*i+1..2*i+2];
        let pos1 = position_in_vector(&vec_char, char1) as u8;
        let pos2 = position_in_vector(&vec_char, char2) as u8;
        let epos : u8 = pos1 + 16 * pos2;
        e_vec.push(epos);
    }
    e_vec
}


pub fn vecu8_to_string(vin: Vec<u8>) -> String {
    let len=vin.len();
    let vec_char=vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
    let mut str_o : String = "".to_string();
    for i in 0..len {
        let eval_u8 : u8 = vin[i];
        let res1_u8 = eval_u8 % 16;
        let res1 = res1_u8 as usize;
        let res2 = ((eval_u8 - res1_u8) / 16) as usize;
        str_o += vec_char[res1];
        str_o += vec_char[res2];
    }
    str_o
}
