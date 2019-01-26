extern crate jsonrpc_core;
use std::process;


mod types;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let nb_arg = arguments.len();
    println!("nb_arg={}", nb_arg);
    if nb_arg != 3 {
        println!("Exiting program. It is run as mkb common_init.json local_init.json");
        process::exit(1);
    }
    let strFileCommonInit = &arguments[1];
    let strFileLocalInit = &arguments[2];
    println!("CommonInit = {} LocalInit = {}", strFileCommonInit, strFileLocalInit);

    
    println!("Hello, world!");
}  
