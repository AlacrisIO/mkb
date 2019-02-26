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


mod db;
mod types;
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
    if nb_arg != 3 {
        println!("Exiting program. It is run as mkb common_init.json local_init.json");
        process::exit(1);
    }
    let str_file_common_init = &arguments[1];
    let str_file_local_init = &arguments[2];
    println!("CommonInit = {}     LocalInit = {}", str_file_common_init, str_file_local_init);
    let common_init : types::CommonInit = parsing_input::read_common_init_ci(str_file_common_init);
    println!("We have common_init");

    let local_init : types::LocalInit = parsing_input::read_local_init(str_file_local_init);
    println!("We have local_init");
    
    let database_file : String = local_init.database_file.clone();
    println!("We have database_file = {}", database_file);
    
    let dbe = db::open_database(&database_file);
    println!("We have opened db");

    let tot_mkb : data_structure::TopicAllInfo = Default::default();
        //data_structure::TopicAllInfo{all_account_state:{}};
    
//    let mut allmerkl = merkle_cbt::MerkleTree::<numext_fixed_hash::H256>::new();
//    println!("We have the merkle database");
    
    infinite_loop::inf_loop(dbe, tot_mkb, common_init, local_init);
    println!("Normal termination of the MKB");
}
