//use std::net::{TcpListener, TcpStream};
//use std::thread;



//use std::io::BufReader;

use std::net::SocketAddr;
use std::collections::HashMap;
use std::iter;
use std::env;
use std::io::{BufReader};
use std::sync::{Arc, Mutex};

use futures::Future;



use tokio_codec::BytesCodec;
use tokio::codec::Decoder;
//use tokio_codec::Decoder
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
//use tokio_core::reactor::Core;

use types::*;
use db::*;





pub fn loop_operation(mut dbe: DBE, sing_reg: SingleRegistrar) -> Result<(), Box<std::error::Error>> {
    let str0 : String = sing_reg.ip_addr[0].to_string();
    let str1 : String = sing_reg.ip_addr[1].to_string();
    let str2 : String = sing_reg.ip_addr[2].to_string();
    let str3 : String = sing_reg.ip_addr[3].to_string();
    let str4 : String = sing_reg.port_int.to_string();
    let ip_plus_port : String = str0 + "." + &str1 + "." + &str2 + "." + &str3 + ":" + &str4;
    let addr: SocketAddr = ip_plus_port.parse().expect("Could not parse ip_plus_port as SocketAddr");

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop, so we pass in a handle
    // to our event loop. After the socket's created we inform that we're ready
    // to go and start accepting connections.
    let socket = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` method.
    //
    // This combinator, defined on the `Stream` trait, will allow us to define a
    // computation to happen for all items on the stream (in this case TCP
    // connections made to the server).  The return value of the `for_each`
    // method is itself a future representing processing the entire stream of
    // connections, and ends up being our server.
    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // Once we're inside this closure this represents an accepted client
            // from our server. The `socket` is the client connection (similar to
            // how the standard library operates).
            //
            // We're parsing each socket with the `BytesCodec` included in `tokio_io`,
            // and then we `split` each codec into the reader/writer halves.
            //
            // See https://docs.rs/tokio-codec/0.1/src/tokio_codec/bytes_codec.rs.html
            let framed = BytesCodec::new().framed(socket);
            let (writer, reader) = framed.split();


            let processor = move || {
                let reader_b = BufReader::new(reader);
                return future::ok(());
            };



            
//                .fold(Vec::<u8>::new(), |acc, x| [acc, x.to_vec()].concat());
            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // This function will transfer ownership of the future (`msg` in this
            // case) to the Tokio runtime thread pool that. The thread pool will
            // drive the future to completion.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.
            tokio::spawn(processor)
        });

    // And finally now that we've define what our server is, we run it!
    //
    // This starts the Tokio runtime, spawns the server task, and blocks the
    // current thread until all tasks complete execution. Since the `done` task
    // never completes (it just keeps accepting sockets), `tokio::run` blocks
    // forever (until ctrl-c is pressed).
    tokio::run(done);
    Ok(())
}
