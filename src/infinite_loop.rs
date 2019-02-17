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



pub fn inf_loop(mut dbe: DBE, common_init: CommonInit, local_init: LocalInit)
{
//    let server_handle : Arc<i32>;
//    let server_handle : Arc<Mutex<i32>>;
//    let server_handle : Arc<Mutex<Option<i32>>>;
//    let server_handle : Arc<Mutex<Option<ServerHandle>>>;
    //    let for_io = server_handle.clone();

    let lk = Arc::new(Mutex::<DBE>::new(dbe));
    let process_request = move |esumreq: SumTypeRequest| {
        let mut w : std::sync::MutexGuard<DBE> = lk.lock().unwrap();
        database_update(w, esumreq);
    };
    let process_request_0 = process_request.clone();
    let process_request_1 = process_request.clone();
    let process_request_2 = process_request.clone();
    let process_request_3 = process_request.clone();
    let process_request_4 = process_request.clone();
    let process_request_5 = process_request.clone();
    
    
    let mut io = IoHandler::new();
    io.add_method("terminate", |_: Params| {
        println!("Processing a terminate command");
        Ok(Value::String("We are trying to exit from the terminate".into()))
    });
    io.add_method("add_account", move |params: Params| {
        println!("Processing a add_account command");
        match params.parse::<AccountInfo>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Accountinfo(eval);
                process_request_0(esumreq);
                return Ok(Value::String("adding account to the system".into()));
            },
            _ => Ok(Value::String("failed adding account".into())),
        }
    });
    io.add_method("deposit", move |params: Params| {
        println!("Processing a deposit command");
        match params.parse::<DepositRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Depositrequest(eval);
                process_request_1(esumreq);
                return Ok(Value::String("deposit operation".into()));
            },
            _ => Ok(Value::String("failed deposit operation".into())),
        }
    });
    io.add_method("payment", move |params: Params| {
        println!("Processing a payment command");
        match params.parse::<PaymentRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Paymentrequest(eval);
                process_request_2(esumreq);
                return Ok(Value::String("payment operation".into()));
            },
            _ => Ok(Value::String("failed payment operation".into())),
        }
    });
    io.add_method("withdrawal", move |params: Params| {
        println!("Processing a withdrawal command");
        match params.parse::<WithdrawRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Withdrawrequest(eval);
                process_request_3(esumreq);
                return Ok(Value::String("withdrawal operation".into()));
            },
            _ => Ok(Value::String("failed withdrawal operation".into())),
        }
    });
    io.add_method("send_data", move |params: Params| {
        println!("Processing a send_data command");
        match params.parse::<SendDataRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Senddatarequest(eval);
                process_request_4(esumreq);
                return Ok(Value::String("send data operation".into()));
            },
            _ => Ok(Value::String("failed send data operation".into())),
        }
    });
    io.add_method("get_data", move |params: Params| {
        println!("Processing a get_data command");
        match params.parse::<GetDataRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Getdatarequest(eval);
                process_request_5(esumreq);
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

