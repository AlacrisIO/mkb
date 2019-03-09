//use std::process;
use jsonrpc_core::*;
//use jsonrpc_core::futures::Future;
//use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use jsonrpc_core::{Error as JsonRpcError};
use secp256k1::{Secp256k1, Message};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use jsonrpc_http_server::{ServerBuilder};

//use types;
use types::*;
use type_init::*;
use db::*;
use data_structure::*;
use gossip_protocol::*;






pub fn inf_loop(dbe: DBE, tot_mkb: TopicAllInfo, common_init: CommonInitFinal, local_init: LocalInitFinal)
{
//    let server_handle : Arc<i32>;
//    let server_handle : Arc<Mutex<i32>>;
//    let server_handle : Arc<Mutex<Option<i32>>>;
//    let server_handle : Arc<Mutex<Option<ServerHandle>>>;
    //    let for_io = server_handle.clone();
    
//    let server_handle = Arc<Mutex<Option<ServerHandle>>>;
//    let server_handle = Arc::new(Mutex::<i32>::new(0));
//    let server_handle : Arc<Mutex<Option<jsonrpc_http_server::ServerHandler>>> = Default::default();

    let common_init_0 = common_init.clone();
    let common_init_1 = common_init.clone();
    let common_init_2 = common_init.clone();
    //
    let my_reg = get_registrar_by_address(local_init.address, &common_init_0).expect("Failed to find registrar");
    let my_reg_0 = my_reg.clone();
    let my_reg_1 = my_reg.clone();
    let my_reg_2 = my_reg.clone();
    let secret_key_copy = local_init.secret_key.clone();

    let lk_dbe = Arc::new(Mutex::<DBE>::new(dbe));
    let lk_mkb = Arc::new(Mutex::<TopicAllInfo>::new(tot_mkb));
    let lk_mkb_0 = lk_mkb.clone();
    let lk_mkb_1 = lk_mkb.clone();
    let lk_mkb_2 = lk_mkb.clone();
    let lk_mkb_3 = lk_mkb.clone();
    let lk_mkb_4 = lk_mkb.clone();
    let sgp = compute_simple_gossip_protocol(&common_init_0, my_reg.address.clone());
    let sgp_0 = sgp.clone();
    let sgp_1 = sgp.clone();

    let complete_process_request = move |esumreq: SumTypeRequest| -> Result<serde_json::Value> {
        println!("complete_process_request, step 1");
        let w_dbe : std::sync::MutexGuard<DBE> = lk_dbe.lock().unwrap();
        println!("complete_process_request, step 2");
        let w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_0.lock().unwrap();
        println!("complete_process_request, step 3");
        let emerkl = get_signature(w_mkb, &my_reg_0, esumreq.clone());
        println!("complete_process_request, step 4");
        if emerkl.result == false {
            return Err(JsonRpcError::invalid_params(emerkl.text));
        }
        println!("complete_process_request, step 5");
        let test = check_mkb_operation(common_init_0.clone(), sgp.clone(), esumreq.clone());
        println!("complete_process_request, step 6");
        if test == false {
            return Err(JsonRpcError::invalid_params("Error with the other registrars".to_string()));
        }
        println!("complete_process_request, step 7");
        database_update(w_dbe, esumreq.clone());
        println!("complete_process_request, step 8");
        match get_topic(&esumreq.clone()) {
            Some(etopic) => {
                let w_mkb_3 : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_1.lock().unwrap();
                send_info_to_registered(w_mkb_3, &etopic, &esumreq);
            },
            None => {},
        }
        println!("complete_process_request, step 9");
        Ok(Value::String("Operation done correcly".into()))
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


    let request_data = move |topic: String, name: String| -> Result<serde_json::Value> {
        let w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_2.lock().unwrap();
        match query_info(w_mkb, topic, name) {
            Ok(eval) => Ok(Value::String(serde_json::to_string(&eval).unwrap())),
            Err(e) => Err(JsonRpcError::invalid_params(e)),
        }
    };
    let get_list_registrar = move || -> Result<serde_json::Value> {
        let estr = serde_json::to_string(&retrieve_complete_list_registrar(common_init_1.clone())).unwrap();
        return Ok(Value::String(estr));
    };
    let get_topic_info = move |topic: String| -> Result<serde_json::Value> {
        let w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_3.lock().unwrap();
        let eval = get_topic_info_wmkb(w_mkb, &my_reg_2, &topic);
        match eval {
            Some(eval_b) => Ok(Value::String(serde_json::to_string(&eval_b).unwrap())),
            None => Err(JsonRpcError::invalid_params("not present topic".to_string())),
        }
    };
    let get_topic_info_0 = get_topic_info.clone();
    let get_topic_info_1 = get_topic_info.clone();
    let get_topic_info_sgp = move |topic: String| -> Result<serde_json::Value> {
        match get_topic_info_sgp_kernel(sgp_1.clone(), topic) {
            None => Err(JsonRpcError::invalid_params("Could not find topic in list".to_string())),
            Some(eval_c) => {
                return Ok(Value::String(serde_json::to_string(&eval_c).unwrap()));
            },
        }
    };

    
    
    let signature_oper_secp256k1 = move |estr: &String | -> SignedString {
        println!("signature_oper_secp256k1, step 1, estr={}", estr);
        let estr_u8 : &[u8] = estr.as_bytes();
        println!("signature_oper_secp256k1, step 2, estr_u8={:?}", estr_u8);
        let len=estr_u8.len();
        println!("signature_oper_secp256k1, step 3, len={}", len);
        let estr_u8_b = get_vector_len_thirtytwo(estr_u8);
        println!("signature_oper_secp256k1, step 4, estr_u8_b={:?}", estr_u8_b);
        let len_b=estr_u8_b.len();
        println!("signature_oper_secp256k1, step 5, len_b={}", len_b);
        let estr_u8_b_ref : &[u8] = &estr_u8_b;
        let secp = Secp256k1::new();
        println!("signature_oper_secp256k1, step 6");
        let message = Message::from_slice(estr_u8_b_ref).expect("Error in creation of message");
        println!("signature_oper_secp256k1, step 7");
        
        let sig : secp256k1::Signature = secp.sign(&message, &secret_key_copy);
        println!("signature_oper_secp256k1, step 8");
        let sig_vec : Vec<u8> = secp256k1::Signature::serialize_der(&sig);
        println!("signature_oper_secp256k1, step 9, sig_vect={:?}", sig_vec);
        println!("signature_oper_secp256k1, step 9, |sig_vect|={}", sig_vec.len());
        
        SignedString {result: estr.to_string(), sig: sig_vec}
    };
    let signature_oper_secp256k1_0 = signature_oper_secp256k1.clone();
    let signature_oper_secp256k1_1 = signature_oper_secp256k1.clone();

        

    
    

    let fct_parsing_error = |e : jsonrpc_core::Error, oper: String| {
        println!("Error during parsing {:?}", e);
        let str0 = "parsing error for ".to_string();
        let str1 = " operation".to_string();
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
            Err(e) => fct_parsing_error(e, "add_account".to_string()),
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
            Err(e) => fct_parsing_error(e, "add_account".to_string()),
        }
    });

    io.add_method("deposit", move |params: Params| {
        println!("Processing a deposit command");
        match params.parse::<DepositRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Depositrequest(eval);
                return process_request_2(esumreq);
            },
            Err(e) => fct_parsing_error(e, "deposit".to_string()),
        }
    });
    io.add_method("payment", move |params: Params| {
        println!("Processing a payment command");
        match params.parse::<PaymentRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Paymentrequest(eval);
                return process_request_3(esumreq);
            },
            Err(e) => fct_parsing_error(e, "deposit".to_string()),
        }
    });
    io.add_method("withdrawal", move |params: Params| {
        println!("Processing a withdrawal command");
        match params.parse::<WithdrawRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Withdrawrequest(eval);
                return process_request_4(esumreq);
            },
            Err(e) => fct_parsing_error(e, "withdrawal".to_string()),
        }
    });
    io.add_method("send_data", move |params: Params| {
        println!("Processing a send_data command");
        match params.parse::<SendDataRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Senddatarequest(eval);
                return process_request_5(esumreq);
            },
            Err(e) => fct_parsing_error(e, "send_data".to_string()),
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
            Err(e) => fct_parsing_error(e, "get_info".to_string()),
        }
    });
    io.add_method("add_subscriber", move |params: Params| {
        println!("Processing a add subscriber request");
        match params.parse::<AddSubscriber>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Addsubscriber(eval);
                return process_request_6(esumreq);
            },
            Err(e) => fct_parsing_error(e, "add_subscriber".to_string()),
        }
    });
    io.add_method("remove_subscriber", move |params: Params| {
        println!("Processing a remove subscriber request");
        match params.parse::<RemoveSubscriber>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Removesubscriber(eval);
                return process_request_7(esumreq);
            },
            Err(e) => fct_parsing_error(e, "add_subscriber".to_string()),
        }
    });
    io.add_method("add_registrar", move |params: Params| {
        println!("Processing a add registrar request");
        match params.parse::<AddRegistrar>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Addregistrar(eval);
                return process_request_8(esumreq);
            },
            Err(e) => fct_parsing_error(e, "add_registrar".to_string()),
        }
    });
    io.add_method("remove_registrar", move |params: Params| {
        println!("Processing a remove registrar request");
        match params.parse::<RemoveRegistrar>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Removeregistrar(eval);
                return process_request_9(esumreq);
            },
            Err(e) => fct_parsing_error(e, "remove_registrar".to_string()),
        }
    });
    io.add_method("get_list_registrars", move |params: Params| {
        println!("Providing the list of registrars");
        match params.parse::<ListRegistrar>() {
            Ok(_) => {
                return get_list_registrar();
            },
            Err(e) => fct_parsing_error(e, "get_list_registrars".to_string()),
        }
    });
    io.add_method("find_topic_info", move |params: Params| {
        println!("Providing information of the topic");
        match params.parse::<RequestInfoTopic>() {
            Ok(eval) => {
                let res = get_topic_info_0(eval.topic.clone());
                match res {
                    Ok(eval_b) => {
                        return Ok(eval_b);
                    }
                    Err(_) => {
                        get_topic_info_sgp(eval.topic)
                    },
                }
            },
            Err(e) => fct_parsing_error(e, "find_topic_info".to_string()),
        }
    });
    io.add_method("retrieve_proof", move |_: Params| {
        Ok(Value::String("retrieve_proof operation".into()))
    });
    //
    // internal operation of the system
    //
    io.add_method("internal_check", move |params: Params| {
        println!("Doing an internal check");
        let fct_signature = move |emer: &MKBoperation| -> Result<serde_json::Value> {
            match emer.result {
                true => {
                    let estr = serde_json::to_string(emer).unwrap();
                    let str_sig = signature_oper_secp256k1_0(&estr);
                    let estr_b = serde_json::to_string(&str_sig).unwrap();
                    return Ok(Value::String(estr_b));
                },
                _ => Err(JsonRpcError::invalid_params("internal_check operation failed".to_string())),
            }
        };
        println!("fct_signature is defined");
        match params.parse::<MessageTransRed>() {
            Ok(eval) => {
                println!("parsing eval, step 1");
                let esumreq = serde_json::from_str(&eval.message).unwrap();
                println!("parsing eval, step 2");
                let w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_4.lock().unwrap();
                let emerkl = get_signature(w_mkb, &my_reg, esumreq);
                println!("parsing eval, step 3");
                return fct_signature(&emerkl);
            },
            Err(e) => fct_parsing_error(e, "internel_check".to_string()),
        }
    });
    io.add_method("internal_request_for_topic_info", move |params: Params| {
        println!("Providing information of the topic");
        match params.parse::<RequestInfoTopic>() {
            Ok(eval) => {
                return get_topic_info_1(eval.topic);
            },
            Err(e) => fct_parsing_error(e, "internal_request_for_topic_info".to_string()),
        }
    });

    
    //
    let my_hostname = IpAddr::V4(Ipv4Addr::new(my_reg_1.ip_addr[0], my_reg_1.ip_addr[1], my_reg_1.ip_addr[2], my_reg_1.ip_addr[3]));
    println!("We have the hostname");
    let socket = SocketAddr::new(my_hostname, my_reg_1.port);
    println!("We have the socket");
    //
    let server = ServerBuilder::new(io)
        .start_http(&socket)
        .expect("Server must start with no issues");
    
    println!("Before server.wait");
    server.wait()
}
