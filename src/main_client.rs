//extern crate jsonrpc_core;
use std::process;
//use std::error::Error;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate jsonrpc_core;
extern crate jsonrpc_pubsub;
extern crate jsonrpc_http_server;
//#[macro_use]
//extern crate jsonrpc_macros;
//extern crate jsonrpc_tcp_server;

extern crate rocksdb;

extern crate multihash;
extern crate chrono;
extern crate secp256k1;

// The ed25519 cryptography. VRF depends on it.
extern crate ed25519_dalek;

// We apparently need to use tokio in order to have
// asynchronous transmissions.
//extern crate tokio;
//extern crate tokio_codec;
//extern crate futures;
//extern crate tokio_core;
//extern crate tokio_io;

#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

//The Merkle trees from CBT.
extern crate merkle_cbt;
extern crate numext_fixed_hash;
    
//big integers
//#![cfg(feature = "serde")]
extern crate num_bigint;
extern crate num_traits;
extern crate num_derive;



//#[macro_use]
//extern crate log;
//use log::{info, trace};


mod types;
mod type_init;
mod type_hash;
mod parsing_input;
mod infinite_loop;
mod gossip_protocol;
mod data_structure;

//mod rpc_server;
//mod rpc_client;

fn main() {
    println!("Beginning MKB");

    let arguments: Vec<String> = std::env::args().collect();
    let nb_arg = arguments.len();
    println!("nb_arg={}", nb_arg);
    if nb_arg != 2 {
        println!("Exiting program. It is run as mkb common_init.json local_init.json");
        process::exit(1);
    }
    let str_file_client_init = &arguments[1];
    println!("ClientInit = {}", str_file_client_init);
    let client_init : type_init::CommonInitFinal = parsing_input::read_client_init(str_file_client_init);
    println!("We have common_init");
    
    infinite_loop::inf_loop_client(common_init, local_init);
    println!("Normal termination of the MKB");
}
