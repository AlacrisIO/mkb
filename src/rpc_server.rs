//use std::net::{TcpListener, TcpStream};
//use std::thread;



use std::io::BufReader;
use std::net::SocketAddr;

use futures::Future;

use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
//use tokio_core::reactor::Core;

use types::*;
use db::*;

pub fn send_message(mesg: Message) -> Result<String,String> {
    let addr: SocketAddr = mesg.ip_plus_port.parse().expect("Could not parse as SocketAddr");
    
    let socket = TcpStream::connect(&addr);
    let request = socket.and_then(|socket| {
        tokio::io::write_all(socket, mesg.message)
    });
    let response = request.and_then(|(socket, _request)| {
        let sock = BufReader::new(socket);
        tokio::io::read_until(sock, b'\n', Vec::new())
    });
    let (_socket, data) = core.run(response).unwrap();
    let str = String::from_utf8_lossy(&data);
    Ok(str);
}



pub fn loop_operation(mut dbe: DBE, sing_reg: SingleRegistrar) {
    let str0 : String = sing_reg.ip_address[0].to_string();
    let str1 : String = sing_reg.ip_address[1].to_string();
    let str2 : String = sing_reg.ip_address[2].to_string();
    let str3 : String = sing_reg.ip_address[3].to_string();
    let str4 : String = sing_reg.port_int.to_string();
    let ip_plus_port : String = str0 + "." + &str1 + "." + &str2 + "." + &str3 + ":" + &str4;
//    let ip_plus_port = sing_reg.ip_address[0].to_string + "."
    //
    let addr: SocketAddr = ip_plus_port.parse().expect("Could not parse as SocketAddr");
    let addr = addr.parse::<SocketAddr>();
    //
    let socket = TcpListener::bind(&addr);
    //
    let srv = socket.incoming()
        .map_err(|e| {println!("failed to accept socket; error = {:?}", e); e})
        .for_each(move |stream| {
            // The client's socket address
            let addr = stream.peer_addr();
            //
            // One handle for reading, one for writing.
            let (reader, writer) = stream.split();
//            let reader_b = BufReader::new(reader);
//            let reader_c = io::read_until(reader, Vec::new());
            //
            let processor = reader
                .for_each(|bytes| {
                    println!("bytes: {:?}", bytes);
                    Ok(())
                })
                .and_then(|()| {
                    println!("Socket received FIN packet and closed connection");
                    Ok(())
                })
                .or_else(|err| {
                    println!("Socket closed with error: {:?}", err);
                    Err(err)
                })
                .then(|result| {
                    println!("Socket closed with result: {:?}", result);
                    
                    Ok(())
                });
            
            // 
            tokio::spawn(processor)
        });
    tokio::run(srv);
    Ok(());
}

