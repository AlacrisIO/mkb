//extern crate jsonrpc_core;
use std::process;
//use std::error::Error;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate jsonrpc_core;
extern crate jsonrpc_pubsub;
#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_tcp_server;

extern crate rocksdb;




extern crate log;
use log::{info, trace};


//use serde::{Serialize, Serializer, Deserialize, Deserializer};
//use self::serde::{Deserialize, Serialize};
//use serde_json::Result;



mod db;
mod types;
mod parsing_input;
mod infinite_loop;

fn main() {
    trace!("Beginning MKB");

    let arguments: Vec<String> = std::env::args().collect();
    let nb_arg = arguments.len();
    println!("nb_arg={}", nb_arg);
    if nb_arg != 3 {
        info!("Exiting program. It is run as mkb common_init.json local_init.json");
        process::exit(1);
    }
    let str_file_common_init = &arguments[1];
    let str_file_local_init = &arguments[2];
    trace!("CommonInit = {}     LocalInit = {}", str_file_common_init, str_file_local_init);
    let common_init : types::CommonInit = parsing_input::read_common_init_ci(str_file_common_init);
    let single_ent : types::SingleEnt = parsing_input::read_single_ent(str_file_local_init);
    let database_file = single_ent.database_file;

    let db = db::open_database(&database_file);

    infinite_loop::inf_loop(db, common_init);
    
    println!("Normal termination of the MKB");
}
