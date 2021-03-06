use jsonrpc_core::*;
use std::sync::{Arc, Mutex};
use jsonrpc_core::{Error as JsonRpcError};
use std::net::{SocketAddr};
use jsonrpc_http_server::{ServerBuilder};

use types::*;
use type_init::*;
use db::*;
use data_structure::*;
use gossip_protocol::*;
use type_sign::*;
use common_net::*;


pub fn get_serialization_typeanswer(e_ans: TypeAnswer) -> Result<serde_json::Value> {
    match e_ans.result {
        false => {
            return Ok(Value::String("answer is false".to_string()));
        },
        true => {
            match e_ans.answer {
                SumTypeAnswer::Trivialanswer(_eval) => {
                    return Ok(Value::String(e_ans.text));
                },
                SumTypeAnswer::Mkboperation(emkb) => {
                    let eans_out = SumTypeAnswerOutput { nature: "Mkboperation".to_string(), hash: emkb.hash};
                    return Ok(serde_json::to_value(eans_out).map_err(|_| jsonrpc_core::Error::internal_error())?);
                },
                _ => {},
            }
            let estr = serde_json::to_string(&e_ans.answer).expect("The serialization failed");
            Ok(Value::String(estr))
        },
    }
}




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
    let secret_key_copy = retrieve_secret_key(&local_init.secret_key);

    let lk_dbe = Arc::new(Mutex::<DBE>::new(dbe));
    let lk_mkb = Arc::new(Mutex::<TopicAllInfo>::new(tot_mkb));
    let lk_mkb_3 = lk_mkb.clone();
    let lk_mkb_4 = lk_mkb.clone();
    let sgp = compute_simple_gossip_protocol(&common_init_0, my_reg.address.clone());
    let sgp_1 = sgp.clone();

    let process_request = move |esumreq: SumTypeRequest| -> Result<serde_json::Value> {
        println!("process_request, step 1");
        let w_dbe : std::sync::MutexGuard<DBE> = lk_dbe.lock().unwrap();
        println!("process_request, step 2");
        let mut w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb.lock().unwrap();
        println!("process_request, step 3");
        let res_oper = process_request_kernel(&mut w_mkb, &my_reg_0.clone(), esumreq.clone(), sgp.clone(), common_init_0.clone());
        match res_oper {
            Err(e) => {
                return Err(JsonRpcError::invalid_params(e));
            },
            Ok(eval) => {
                println!("process_request, step 7");
                database_update(w_dbe, esumreq.clone());
                println!("process_request, step 8");
                match get_topic_export_subscriber(&esumreq.clone()) {
                    Some(etopic) => {
                        send_info_to_registered(w_mkb, &etopic, &esumreq);
                    },
                    None => {},
                }
                println!("process_request, step 9");
                let estr = get_serialization_typeanswer(eval);
                estr
            },
        }
    };
    let process_request_0  = process_request.clone();
    let process_request_1  = process_request.clone();
    let process_request_2  = process_request.clone();
    let process_request_3  = process_request.clone();
    let process_request_4  = process_request.clone();
    let process_request_5  = process_request.clone();
    let process_request_6  = process_request.clone();
    let process_request_7  = process_request.clone();
    let process_request_8  = process_request.clone();
    let process_request_9  = process_request.clone();
    let process_request_10 = process_request.clone();
    let process_request_11 = process_request.clone();
    let process_request_12 = process_request.clone();
    let process_request_13 = process_request.clone();


    let get_total_list_registrars = move || -> Result<serde_json::Value> {
        let estr = serde_json::to_string(&retrieve_complete_list_registrar(common_init_1.clone())).unwrap();
        return Ok(Value::String(estr));
    };
    let get_registrar_by_name_loc = move |name: String| -> Option<SingleRegistrarFinal> {
        get_registrar_by_name(name, &common_init_2.clone())
    };
    let get_topic_info = move |topic: String| -> Result<serde_json::Value> {
        let w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_3.lock().unwrap();
        let eval = get_topic_info_wmkb(&w_mkb, &my_reg_2, &topic);
        match eval {
            Some(eval_b) => Ok(Value::String(serde_json::to_string(&eval_b).unwrap())),
            None => Err(JsonRpcError::invalid_params("not present topic".to_string())),
        }
    };
    let get_topic_info_sgp = move |topic: String| -> Result<serde_json::Value> {
        match get_topic_info_sgp_kernel(sgp_1.clone(), topic) {
            None => Err(JsonRpcError::invalid_params("Could not find topic in list".to_string())),
            Some(eval_c) => {
                return Ok(Value::String(serde_json::to_string(&eval_c).unwrap()));
            },
        }
    };
    let fct_parsing_error = |e : jsonrpc_core::Error, oper: String| {
        println!("Error during parsing {:?}", e);
        let str0 = "parsing error for ".to_string();
        let str1 = " operation".to_string();
        let str_out = str0 + &oper + &str1;
        return Err(JsonRpcError::invalid_params(str_out));
    };

    let mut io = IoHandler::new();
    io.add_method("mkb_terminate", |_: Params| {
        println!("Processing a terminate command");
        Ok(Value::String("We are trying to exit from the terminate".into()))
    });
    io.add_method("mkb_topic_creation", move |params: Params| {
        println!("Processing a topic_creation request command");
        match params.parse::<TopicDescription>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Topiccreationrequest(eval);
                return process_request_0(esumreq);
            },
            Err(e) => fct_parsing_error(e, "topic_creation".to_string()),
        }
    });
    io.add_method("mkb_add_account", move |params: Params| {
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
    io.add_method("mkb_deposit", move |params: Params| {
        println!("Processing a deposit command");
        match params.parse::<DepositRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Depositrequest(eval);
                return process_request_2(esumreq);
            },
            Err(e) => fct_parsing_error(e, "deposit".to_string()),
        }
    });
    io.add_method("mkb_payment", move |params: Params| {
        println!("Processing a payment command");
        match params.parse::<PaymentRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Paymentrequest(eval);
                return process_request_3(esumreq);
            },
            Err(e) => fct_parsing_error(e, "deposit".to_string()),
        }
    });
    io.add_method("mkb_withdrawal", move |params: Params| {
        println!("Processing a withdrawal command");
        match params.parse::<WithdrawRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Withdrawrequest(eval);
                return process_request_4(esumreq);
            },
            Err(e) => fct_parsing_error(e, "withdrawal".to_string()),
        }
    });
    io.add_method("mkb_send_data", move |params: Params| {
        println!("Processing a send_data command");
        match params.parse::<SendDataRequestInput>() {
            Ok(eval) => {
                let eval_b = SendDataRequest { topic: eval.topic, account_name: eval.account_name,
                                               hash: string_to_vecu8(eval.hash),
                                               data: eval.data };
                let esumreq = SumTypeRequest::Senddatarequest(eval_b);
                return process_request_5(esumreq);
            },
            Err(e) => fct_parsing_error(e, "send_data".to_string()),
        }
    });
    io.add_method("mkb_send_key_value", move |params: Params| {
        println!("Processing a send_key_value command");
        match params.parse::<SendKeyValueRequest>() {
            Ok(eval) => {
                let eval_b = SendKeyValueRequest { topic: eval.topic, account_name: eval.account_name,
                                                   key: eval.key, value: eval.value };
                let esumreq = SumTypeRequest::Sendkeyvaluerequest(eval_b);
                return process_request_6(esumreq);
            },
            Err(e) => fct_parsing_error(e, "send_data_key_value".to_string()),
        }
    });
    io.add_method("mkb_get_key_value", move |params: Params| {
        println!("Processing a get_data_key_value command");
        match params.parse::<GetKeyValueRequest>() {
            Ok(eval) => {
                let eval_b = GetKeyValueRequest { topic: eval.topic, account_name: eval.account_name,
                                                  key: eval.key};
                let esumreq = SumTypeRequest::Getkeyvaluerequest(eval_b);
                return process_request_7(esumreq);
            },
            Err(e) => fct_parsing_error(e, "get_key_value".to_string()),
        }
    });
    io.add_method("mkb_get_from_latest", move |params: Params| {
        println!("Processing a get_from_latest");
        match params.parse::<GetInfoRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Getlatestentry(eval);
                return process_request_8(esumreq);
            },
            Err(e) => fct_parsing_error(e, "get_from_latest".to_string()),
        }
    });
    io.add_method("mkb_get_from_triple", move |params: Params| {
        println!("Processing a get_from_triple");
        match params.parse::<TripleRequest>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Triplerequest(eval);
                return process_request_9(esumreq);
            },
            Err(e) => fct_parsing_error(e, "get_info".to_string()),
        }
    });
    io.add_method("mkb_add_subscriber", move |params: Params| {
        println!("Processing a add subscriber request");
        match params.parse::<AddSubscriber>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Addsubscriber(eval);
                return process_request_10(esumreq);
            },
            Err(e) => fct_parsing_error(e, "add_subscriber".to_string()),
        }
    });
    io.add_method("mkb_remove_subscriber", move |params: Params| {
        println!("Processing a remove_subscriber request");
        match params.parse::<RemoveSubscriber>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Removesubscriber(eval);
                return process_request_11(esumreq);
            },
            Err(e) => fct_parsing_error(e, "add_subscriber".to_string()),
        }
    });
    io.add_method("mkb_add_registrar", move |params: Params| {
        println!("Processing a add_registrar request");
        match params.parse::<AddRegistrarInput>() {
            Ok(eval) => {
                match get_registrar_by_name_loc(eval.registrar_name) {
                    None => Err(JsonRpcError::invalid_params("wrong registrar name".to_string())),
                    Some(ereg) => {
                        let eval_b = AddRegistrar { topic: eval.topic, registrar_address: ereg.address};
                        let esumreq = SumTypeRequest::Addregistrar(eval_b);
                        return process_request_12(esumreq);
                    }
                }
            },
            Err(e) => fct_parsing_error(e, "add_registrar".to_string()),
        }
    });
    io.add_method("mkb_remove_registrar", move |params: Params| {
        println!("Processing a remove_registrar request");
        match params.parse::<RemoveRegistrar>() {
            Ok(eval) => {
                let esumreq = SumTypeRequest::Removeregistrar(eval);
                return process_request_13(esumreq);
            },
            Err(e) => fct_parsing_error(e, "remove_registrar".to_string()),
        }
    });
    io.add_method("mkb_get_total_list_registrars", move |params: Params| {
        println!("Processing a get_total_list_registrars command");
        match params.parse::<TotalListRegistrar>() {
            Ok(_) => {
                return get_total_list_registrars();
            },
            Err(e) => fct_parsing_error(e, "get_total_list_registrars".to_string()),
        }
    });
    io.add_method("mkb_find_topic_info", move |params: Params| {
        println!("Providing information of the topic");
        match params.parse::<RequestInfoTopic>() {
            Ok(eval) => {
                let res = get_topic_info(eval.topic.clone());
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
    io.add_method("mkb_retrieve_proof", move |_: Params| {
        Ok(Value::String("retrieve_proof operation".into()))
    });
    //
    // internal operation of the system
    //
    io.add_method("mkb_internal_operation", move |params: Params| {
        println!("Doing an internal check");
        let fct_signature = move |emer: &TypeAnswer| -> Result<serde_json::Value> {
            match emer.result {
                true => {
                    let estr = serde_json::to_string(emer).unwrap();
                    let str_sig = signature_oper_secp256k1(secret_key_copy, &estr);
                    let estr_b = serde_json::to_string(&str_sig).unwrap();
                    return Ok(Value::String(estr_b));
                },
                _ => Err(JsonRpcError::invalid_params("mkb_internal_operation failed e={}".to_string())),
            }
        };
        println!("fct_signature is defined");
        match params.parse::<MessageTransRed>() {
            Ok(eval) => {
                println!("parsing eval, step 1");
                println!("eval.message={:?}", eval.message);
                let esumreq : SumTypeRequest = serde_json::from_str(&eval.message).unwrap();
                println!("parsing eval, step 2");
                let mut w_mkb : std::sync::MutexGuard<TopicAllInfo> = lk_mkb_4.lock().unwrap();
                let res_oper = process_operation(&mut w_mkb, common_init.clone(), &my_reg, esumreq);
                println!("parsing eval, step 3");
                return fct_signature(&res_oper);
            },
            Err(e) => fct_parsing_error(e, "internel_check".to_string()),
        }
    });
    //
    let my_hostname = retrieve_v4_addr(my_reg_1.ip_addr);
    let socket = SocketAddr::new(my_hostname, my_reg_1.port);
    println!("We have the socket");
    //
    let server = ServerBuilder::new(io)
        .start_http(&socket)
        .expect("Server must start with no issues");
    println!("Before server.wait");
    server.wait()
}
