//use std::process;
use jsonrpc_core::*;
//use jsonrpc_core::futures::Future;

use rocksdb::DB;

use types;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use jsonrpc_http_server::{ServerBuilder};
//use jsonrpc_minihttp_server::jsonrpc_core::*;


//use jsonrpc_tcp_server::ServerBuilder;
//use jsonrpc_tcp_server::jsonrpc_core::*;

fn get_registrar_by_address(address: String, common_init: types::CommonInit) -> Option<types::SingleRegistrar> {
    for e_rec in common_init.registrars {
        if e_rec.address == address {
            return Some(e_rec);
        }
    }
    None
}



pub fn inf_loop(db: DB, common_init: types::CommonInit, local_init: types::LocalInit)
{
    let mut io = IoHandler::new();
    io.add_method("terminate", |_: Params| {
        println!("Processing a terminate command");
        Ok(Value::String("We are trying to exit from the terminate".into()))
    });
    io.add_method("add_account", |_: Params| {
        println!("Processing a add_account command");
//        Ok(json!({"success": true}))
        Ok(Value::String("adding account to the system".into()))
    });
    io.add_method("deposit", |_: Params| {
        println!("Processing a deposit command");
        Ok(Value::String("deposit operation".into()))
    });
    io.add_method("payment", |_: Params| {
        println!("Processing a payment command");
        Ok(Value::String("payment operation".into()))
    });
    io.add_method("withdrawal", |_: Params| {
        println!("Processing a withdrawal command");
        Ok(Value::String("withdrawal operation".into()))
    });
    io.add_method("send_data", |_: Params| {
        println!("Processing a send_data command");
        Ok(Value::String("send data operation".into()))
    });
    io.add_method("get_data", |_: Params| {
        println!("Processing a get_data command");
        Ok(Value::String("get data operation".into()))
    });
    //
    let my_reg = get_registrar_by_address(local_init.address, common_init).expect("Failed to find registrar");

    let my_hostname = IpAddr::V4(Ipv4Addr::new(my_reg.ip_address[0], my_reg.ip_address[1], my_reg.ip_address[2], my_reg.ip_address[3]));
    println!("We have the hostname");
    let socket = SocketAddr::new(my_hostname, my_reg.port);
    println!("We have the socket");


    //
    let server = ServerBuilder::new(io)
        .start_http(&socket)
        .expect("Server must start with no issues");
    
    server.wait()

    
    
    
}

