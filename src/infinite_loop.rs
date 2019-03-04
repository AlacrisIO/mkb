//use std::process;
use jsonrpc_core::*;
//use jsonrpc_core::futures::Future;
//use std::collections::HashMap;
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
use data_structure::*;
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



pub fn inf_loop(dbe: DBE, tot_mkb: TopicAllInfo, common_init: CommonInit, local_init: LocalInit)
{
//    let server_handle : Arc<i32>;
//    let server_handle : Arc<Mutex<i32>>;
//    let server_handle : Arc<Mutex<Option<i32>>>;
//    let server_handle : Arc<Mutex<Option<ServerHandle>>>;
    //    let for_io = server_handle.clone();
    
//    let server_handle = Arc<Mutex<Option<ServerHandle>>>;
//    let server_handle = Arc::new(Mutex::<i32>::new(0));
//    let server_handle : Arc<Mutex<Option<jsonrpc_http_server::ServerHandler>>> = Default::default();

    
    //
    let my_reg = get_registrar_by_address(local_init.address, &common_init).expect("Failed to find registrar");

    let lk_dbe = Arc::new(Mutex::<DBE>::new(dbe));
    let lk_mkb = Arc::new(Mutex::<TopicAllInfo>::new(tot_mkb));
    let lk_mkb_0 = lk_mkb.clone();
    let lk_mkb_1 = lk_mkb.clone();
    let lk_mkb_2 = lk_mkb.clone();
    let lk_mkb_3 = lk_mkb.clone();
/*    
    let process_request = move |esumreq: SumTypeRequest| {
        let w : std::sync::MutexGuard<DBE> = lk.lock().unwrap();
        
        database_update(w, esumreq);
    };
*/
    let sgp = compute_simple_gossip_protocol(&common_init, my_reg.address);


    let complete_process_request = move |esumreq: SumTypeRequest| -> Result<serde_json::Value> {
        let w_dbe : std::sync::MutexGuard<DBE> = lk_dbe.lock().unwrap();
        let w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_0.lock().unwrap();
        let emerkl = get_signature(w_mkb, esumreq.clone());
        if emerkl.result == false {
            return Err(JsonRpcError::invalid_params("Error with the merkle database".to_string()));
        }
        let test = check_mkb_operation(common_init.clone(), sgp.clone(), esumreq.clone());
        if test == false {
            return Err(JsonRpcError::invalid_params("Error with remote merkle database".to_string()));
        }
        database_update(w_dbe, esumreq.clone());
        match get_topic(&esumreq.clone()) {
            Some(etopic) => {
                let w_mkb_3 : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_3.lock().unwrap();
                send_info_to_registered(w_mkb_3, &etopic, &esumreq);
            },
            None => {},
        }
        Ok(Value::String("Operation done correcly".into()))
    };


    let request_data = move |topic: String, name: String| -> Result<serde_json::Value> {
        let w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_1.lock().unwrap();
        match query_info(w_mkb, topic, name) {
            Ok(eval) => Ok(Value::String(serde_json::to_string(&eval).unwrap())),
            Err(e) => Err(JsonRpcError::invalid_params(e)),
        }
    };


        

    
    
    let process_request_0 = complete_process_request.clone();
    let process_request_1 = complete_process_request.clone();
    let process_request_2 = complete_process_request.clone();
    let process_request_3 = complete_process_request.clone();
    let process_request_4 = complete_process_request.clone();
    let process_request_5 = complete_process_request.clone();
    let process_request_6 = complete_process_request.clone();
    let process_request_7 = complete_process_request.clone();
    let process_request_8 = complete_process_request.clone();
    let process_request_9 = complete_process_request.clone();

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
    io.add_method("topic_creation", move |params: Params| {
        println!("Processing a topic_creation_request command");
        match params.parse::<TopicDescription>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Topiccreationrequest(eval);
                return process_request_0(esumreq);
            },
            Err(e) => fct_error(e, "add_account".to_string()),
        }
    });
    io.add_method("add_account", move |params: Params| {
        println!("Processing a add_account command");
        match params.parse::<AccountInfo>() {
            Ok(eval) => {
                println!("add_account, step 1");
                let esumreq = SumTypeRequest::Accountinfo(eval);
                println!("add_account, step 2");
                return process_request_1(esumreq);
            },
            Err(e) => fct_error(e, "add_account".to_string()),
        }
    });

    io.add_method("deposit", move |params: Params| {
        println!("Processing a deposit command");
        match params.parse::<DepositRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Depositrequest(eval);
                return process_request_2(esumreq);
            },
            Err(e) => fct_error(e, "deposit".to_string()),
        }
    });
    io.add_method("payment", move |params: Params| {
        println!("Processing a payment command");
        match params.parse::<PaymentRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Paymentrequest(eval);
                return process_request_3(esumreq);
            },
            Err(e) => fct_error(e, "deposit".to_string()),
        }
    });
    io.add_method("withdrawal", move |params: Params| {
        println!("Processing a withdrawal command");
        match params.parse::<WithdrawRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Withdrawrequest(eval);
                return process_request_4(esumreq);
            },
            Err(e) => fct_error(e, "withdrawal".to_string()),
        }
    });
    io.add_method("send_data", move |params: Params| {
        println!("Processing a send_data command");
        match params.parse::<SendDataRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Senddatarequest(eval);
                return process_request_5(esumreq);
            },
            Err(e) => fct_error(e, "send_data".to_string()),
        }
    });
    io.add_method("get_info", move |params: Params| {
        println!("Processing a get_data command");
        match params.parse::<GetInfoRequest>() {
            Ok(eval) => {
                let e_topic = eval.topic;
                let e_account_name = eval.account_name;
                return request_data(e_topic, e_account_name);
            },
            Err(e) => fct_error(e, "get_info".to_string()),
        }
    });
    io.add_method("add_subscriber", move |params: Params| {
        println!("Processing a add subscriber request");
        match params.parse::<AddSubscriber>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Addsubscriber(eval);
                return process_request_6(esumreq);
            },
            Err(e) => fct_error(e, "add_subscriber".to_string()),
        }
    });
    io.add_method("remove_subscriber", move |params: Params| {
        println!("Processing a remove subscriber request");
        match params.parse::<RemoveSubscriber>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Removesubscriber(eval);
                return process_request_7(esumreq);
            },
            Err(e) => fct_error(e, "add_subscriber".to_string()),
        }
    });
    io.add_method("add_registrar", move |params: Params| {
        println!("Processing a add registrar request");
        match params.parse::<AddRegistrar>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Addregistrar(eval);
                return process_request_8(esumreq);
            },
            Err(e) => fct_error(e, "add_registrar".to_string()),
        }
    });
    io.add_method("remove_registrar", move |params: Params| {
        println!("Processing a remove registrar request");
        match params.parse::<RemoveRegistrar>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Removeregistrar(eval);
                return process_request_9(esumreq);
            },
            Err(e) => fct_error(e, "remove_registrar".to_string()),
        }
    });

    
    io.add_method("internal_check", move |params: Params| {
        println!("Doing an internal check");
        fn fct_signature(emer: &MKBoperation) -> Result<serde_json::Value> {
            match emer.result {
                true => {
                    let estr = serde_json::to_string(emer).unwrap();
                    return Ok(Value::String(estr));
                },
                _ => Err(JsonRpcError::invalid_params("internal_check operation failed".to_string())),
            }
        };
        println!("fct_signature is defined");
        match params.parse::<MessageRed>() {
            Ok(eval) => {
                println!("parsing eval, step 1");
                let esumreq = serde_json::from_str(&eval.message).unwrap();
                println!("parsing eval, step 2");
                let w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_2.lock().unwrap();
                let emerkl = get_signature(w_mkb, esumreq);
                println!("parsing eval, step 3");
                return fct_signature(&emerkl);
            },
            Err(e) => fct_error(e, "internel_check".to_string()),
        }
    });

    io.add_method("retrieve_proof", move |_: Params| {
        Ok(Value::String("retrieve_proof operation".into()))
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
