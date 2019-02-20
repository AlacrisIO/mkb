//use std::process;
use jsonrpc_core::*;
//use jsonrpc_core::futures::Future;
use std::sync::{Arc, Mutex};
use jsonrpc_core::{Error as JsonRpcError};

//use std::sync::RwLock;

//use futures::Future;
//use futures::future::{self, Either};
//use futures_cpupool::CpuPool;

//use rocksdb::DB;
//use types;
use types::*;
use db::*;
use merkle_data_tree::*;
use gossip_protocol::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use jsonrpc_http_server::{ServerBuilder};
//use jsonrpc_client_http::HttpTransport;



//use jsonrpc_tcp_server::ServerBuilder;
//use jsonrpc_tcp_server::jsonrpc_core::*;







fn get_registrar_by_address(address: String, common_init: &CommonInit) -> Option<SingleRegistrar> {
    for e_rec in &common_init.registrars {
        if e_rec.address == address {
            return Some(e_rec.clone());
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

    
    //
    let my_reg = get_registrar_by_address(local_init.address, &common_init).expect("Failed to find registrar");

    let lk = Arc::new(Mutex::<DBE>::new(dbe));
    let check_correctness = move |esumreq: SumTypeRequest| {
        true
    };
    let check_correctness_0 = check_correctness.clone();
    let check_correctness_1 = check_correctness.clone();
/*    
    let process_request = move |esumreq: SumTypeRequest| {
        let w : std::sync::MutexGuard<DBE> = lk.lock().unwrap();
        
        database_update(w, esumreq);
    };
*/
    let sgp = compute_simple_gossip_protocol(&common_init, my_reg.address);


    let complete_process_request = move |esumreq: SumTypeRequest| -> Result<serde_json::Value> {
        let w : std::sync::MutexGuard<DBE> = lk.lock().unwrap();
        let emerkl = get_signature(esumreq.clone());
        if emerkl.result == false {
            return Err(JsonRpcError::invalid_params("Error with the merkle database".to_string()));
        }
        let test = check_mkb_operation(common_init.clone(), sgp.clone(), esumreq.clone());
        if test == false {
            return Err(JsonRpcError::invalid_params("Error with remote merkle database".to_string()));
        }
        database_update(w, esumreq);
        Ok(Value::String("Operation wetn correcly".into()))
    };



    
    
    let process_request_0 = complete_process_request.clone();
    let process_request_1 = complete_process_request.clone();
    let process_request_2 = complete_process_request.clone();
    let process_request_3 = complete_process_request.clone();
    let process_request_4 = complete_process_request.clone();
    let process_request_5 = complete_process_request.clone();

    let fct_error = |e : jsonrpc_core::Error, oper: String| {
        println!("Error during parsing {:?}", e);
        let str0 = "failed".to_string();
        let str1 = "operation".to_string();
        let str_out = str0 + &oper + &str1;
        return Err(JsonRpcError::invalid_params(str_out));
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
                let esumreq = SumTypeRequest::Accountinfo(eval);
                return process_request_0(esumreq);
            },
            Err(e) => fct_error(e, "add_account".to_string()),
        }
    });

    io.add_method("deposit", move |params: Params| {
        println!("Processing a deposit command");
        match params.parse::<DepositRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Depositrequest(eval);
                return process_request_1(esumreq);
            },
            Err(e) => fct_error(e, "deposit".to_string()),
        }
    });
    io.add_method("payment", move |params: Params| {
        println!("Processing a payment command");
        match params.parse::<PaymentRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Paymentrequest(eval);
                return process_request_2(esumreq);
            },
            Err(e) => fct_error(e, "deposit".to_string()),
        }
    });
    io.add_method("withdrawal", move |params: Params| {
        println!("Processing a withdrawal command");
        match params.parse::<WithdrawRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Withdrawrequest(eval);
                return process_request_3(esumreq);
            },
            Err(e) => fct_error(e, "withdrawal".to_string()),
        }
    });
    io.add_method("send_data", move |params: Params| {
        println!("Processing a send_data command");
        match params.parse::<SendDataRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Senddatarequest(eval);
                return process_request_4(esumreq);
            },
            Err(e) => fct_error(e, "send_data".to_string()),
        }
    });
    io.add_method("get_data", move |params: Params| {
        println!("Processing a get_data command");
        match params.parse::<GetDataRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Getdatarequest(eval);
                return process_request_5(esumreq);
            },
            Err(e) => fct_error(e, "get_data".to_string()),
        }
    });


    io.add_method("internal_check", move |params: Params| {
        println!("Doing an internal check");
        fn fct_signature(emer: &MerkleVerification) -> Result<serde_json::Value> {
            match emer.result {
                true => {
                    let estr = serde_json::to_string(emer).unwrap();
                    return Ok(Value::String(estr));
                },
                _ => Err(JsonRpcError::invalid_params("internal_check operation failed".to_string())),
            }
        };
        match params.parse::<Message>() {
            Ok(eval) => {
                let esumreq = serde_json::from_str(&eval.message).unwrap();
                let emerkl = get_signature(esumreq);
                return fct_signature(&emerkl);
            },
            Err(e) => fct_error(e, "internel_check".to_string()),
        }
    });
    io.add_method("retrieve_proof", move |params: Params| {
        Ok(Value::String("rerieve_proof operation".into()))
    });

    
    //
    let my_hostname = IpAddr::V4(Ipv4Addr::new(my_reg.ip_address[0], my_reg.ip_address[1], my_reg.ip_address[2], my_reg.ip_address[3]));
    println!("We have the hostname");
    let socket = SocketAddr::new(my_hostname, my_reg.port);
    println!("We have the socket");
    //
    let server = ServerBuilder::new(io)
        .start_http(&socket)
        .expect("Server must start with no issues");
    
    println!("Before server.wait");
    server.wait()
}

