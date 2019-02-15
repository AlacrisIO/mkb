//use std::process;
use jsonrpc_core::*;
//use jsonrpc_core::futures::Future;
use std::sync::{Arc, Mutex};

use std::sync::RwLock;

//use futures::Future;
//use futures::future::{self, Either};
//use futures_cpupool::CpuPool;

use rocksdb::DB;
//use types;
use types::*;
use db::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use jsonrpc_http_server::{ServerBuilder};
use jsonrpc_client_http::HttpTransport;



//use jsonrpc_tcp_server::ServerBuilder;
//use jsonrpc_tcp_server::jsonrpc_core::*;







fn get_registrar_by_address(address: String, common_init: CommonInit) -> Option<SingleRegistrar> {
    for e_rec in common_init.registrars {
        if e_rec.address == address {
            return Some(e_rec);
        }
    }
    None
}

fn database_update(mut w: std::sync::RwLockWriteGuard<DBE>, esumreq: SumTypeRequest) {
    let iter = (*w).iter;
    let iter_str = iter.to_string();
    let value_str : String = serde_json::to_string(&esumreq).unwrap();
    //
    let key_u8_b = iter_str.as_bytes();
    let value_u8_b = value_str.as_bytes();
    (*w).db.put(key_u8_b, value_u8_b).unwrap();
    let iter_inc : i32 = iter + 1;
    (*w).iter = iter_inc;
    
}


pub fn inf_loop(mut dbe: DBE, common_init: CommonInit, local_init: LocalInit)
{
//    let server_handle : Arc<i32>;
//    let server_handle : Arc<Mutex<i32>>;
//    let server_handle : Arc<Mutex<Option<i32>>>;
//    let server_handle : Arc<Mutex<Option<ServerHandle>>>;
    //    let for_io = server_handle.clone();

    let lock = RwLock::<DBE>::new(dbe);
    let process_request = move |esumreq: SumTypeRequest| {
        let mut w : std::sync::RwLockWriteGuard<DBE> = lock.write().unwrap();
        database_update(w, esumreq);
    };


    
    let nb_call : RwLock<i32> = RwLock::new(0);
    let increase_nb_call = move || {
        let mut w = nb_call.write().unwrap();
        *w += 1;
    };

    
    
    let mut io = IoHandler::new();
    io.add_method("terminate", |_: Params| {
        println!("Processing a terminate command");
        Ok(Value::String("We are trying to exit from the terminate".into()))
    });
    io.add_method("add_account", move |params: Params| {
        println!("Processing a add_account command");
        match params.parse::<AccountInfo>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::accountinfo(eval);
                increase_nb_call();
                process_request(esumreq);
//                return future:ok(Value::SymTypeRequest(esumreq));
                return Ok(Value::String("adding account to the system".into()));
            },
            _ => Ok(Value::String("failed adding account".into())),
        }
    });
    io.add_method("deposit", |params: Params| {
        println!("Processing a deposit command");
        match params.parse::<DepositRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::depositrequest(eval);
                return Ok(Value::String("deposit operation".into()));
            },
            _ => Ok(Value::String("failed deposit operation".into())),
        }
    });
    io.add_method("payment", |params: Params| {
        println!("Processing a payment command");
        match params.parse::<PaymentRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::paymentrequest(eval);
                return Ok(Value::String("payment operation".into()));
            },
            _ => Ok(Value::String("failed payment operation".into())),
        }
    });
    io.add_method("withdrawal", |params: Params| {
        println!("Processing a withdrawal command");
        match params.parse::<PaymentRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::paymentrequest(eval);
                return Ok(Value::String("withdrawal operation".into()));
            },
            _ => Ok(Value::String("failed withdrawal operation".into())),
        }
    });
    io.add_method("send_data", |params: Params| {
        println!("Processing a send_data command");
        match params.parse::<SendDataRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::senddatarequest(eval);
                return Ok(Value::String("send data operation".into()));
            },
            _ => Ok(Value::String("failed send data operation".into())),
        }
    });
    io.add_method("get_data", |params: Params| {
        println!("Processing a get_data command");
        match params.parse::<GetDataRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::getdatarequest(eval);
                return Ok(Value::String("get data operation".into()));
            },
            _ => Ok(Value::String("failed get data operation".into())),
        }
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

