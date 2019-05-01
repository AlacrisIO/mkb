use jsonrpc_core::*;
use jsonrpc_core::{Error as JsonRpcError};
use std::net::{SocketAddr};
use jsonrpc_http_server::{ServerBuilder};

//use types;
use types::*;
use type_init::*;
use common_net::*;

jsonrpc_client!(pub struct InternalClient {
    pub fn send_data(&mut self, transmission: String) -> RpcRequest<String>;
});


pub fn send_data_cs(client_init: &ClientInitFinal, esend: SendDataRequest) -> Result<serde_json::Value> {
    let estr = "testr".to_string();
    Ok(Value::String(estr))
}





pub fn inf_loop_client(client_init: ClientInitFinal)
{
    let client_init_0 = client_init.clone();
    let fct_parsing_error = |e : jsonrpc_core::Error, oper: String| {
        println!("Error during parsing {:?}", e);
        let str0 = "parsing error for ".to_string();
        let str1 = " operation".to_string();
        let str_out = str0 + &oper + &str1;
        return Err(JsonRpcError::invalid_params(str_out));
    };

    let process_request = move |esend: SendDataRequest| -> Result<serde_json::Value> {
        send_data_cs(&client_init_0, esend)
    };
    
    
    let mut io = IoHandler::new();
    io.add_method("send_data", move |params: Params| {
        println!("Processing a send_data command");
        match params.parse::<SendDataRequest>() {
            Ok(eval) => {
                return process_request(eval);
            },
            Err(e) => fct_parsing_error(e, "send_data".to_string()),
        }
    });
    io.add_method("registration_info", move |params: Params| {
        println!("Printing from the registered registrar");
        match params.parse::<MessageTransRed>() {
            Ok(eval) => {
                let esumreq : SumTypeRequest = serde_json::from_str(&eval.message).expect("Parsing failed");
                let estr = "parsing eval, step 2".to_string();
                return Ok(Value::String(estr));
            },
            Err(e) => fct_parsing_error(e, "internel_check".to_string()),
        }
    });

    
    //
    let my_hostname = retrieve_v4_addr(client_init.client_ip_addr);
    let socket = SocketAddr::new(my_hostname, client_init.client_port);
    println!("We have the socket");
    //
    let server = ServerBuilder::new(io)
        .start_http(&socket)
        .expect("Server must start with no issues");
    
    println!("Before server.wait");
    server.wait()
}
