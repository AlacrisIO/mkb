//use std::process;
use jsonrpc_core::*;
//use jsonrpc_core::futures::Future;

use rocksdb::DB;

use types;

use log::{info};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use jsonrpc_tcp_server::ServerBuilder;
//use jsonrpc_tcp_server::jsonrpc_core::*;

fn get_registrar_by_address(address: String, common_init: types::CommonInit) -> Option<types::SingleRegistrar> {
    for e_rec in common_init.registrars {
        if e_rec.address == address {
            Some(e_rec);
        }
    }
    None
//    info!("Failed to find the address in the list of registrars");
//    process::exit(1);
}



pub fn inf_loop(db: DB, common_init: types::CommonInit, local_init: types::LocalInit)
{
    let mut io = IoHandler::new();
    io.add_method("terminate", |_: Params| {
        Ok(Value::String("We are trying to exit from the terminate".into()))
    });
    io.add_method("add_account", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("deposit", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("payment", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("withdrawal", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("send_data", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("get_data", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    //
    let my_reg = get_registrar_by_address(local_init.address, common_init).expect("Failed to find registrar");

    let my_hostname = IpAddr::V4(Ipv4Addr::new(my_reg.ip_address[0], my_reg.ip_address[1], my_reg.ip_address[2], my_reg.ip_address[3]));
    info!("We have the hostname");
    let socket = SocketAddr::new(my_hostname, my_reg.port);
    info!("We have the socket");


    //
    let server = ServerBuilder::new(io)
        .start(&socket)
        .expect("Server must start with no issues");
    
    server.wait()

    
    
    
}

