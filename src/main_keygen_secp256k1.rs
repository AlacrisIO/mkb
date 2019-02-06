extern crate secp256k1;

use secp256k1::{Secp256k1, Message};
use secp256k1::rand::OsRng;

fn main()
{
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");

    let sig = secp.sign(&message, &secret_key);
    assert!(secp.verify(&message, &sig, &public_key).is_ok());
}
