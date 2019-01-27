//extern crate jsonrpc_core;
use std::process;
//use std::error::Error;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

//use serde::{Serialize, Serializer, Deserialize, Deserializer};
//use self::serde::{Deserialize, Serialize};
//use serde_json::Result;



mod types;
mod parsing_input;

fn main() {
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
    println!("Hello, world!");
}  
