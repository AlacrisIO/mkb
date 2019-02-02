extern crate jsonrpc_core;
//extern crate jsonrpc_pubsub;
//#[macro_use]
//extern crate jsonrpc_macros;
//extern crate jsonrpc_tcp_server;

use jsonrpc_core;
//use jsonrpc_core::*;
use jsonrpc_core::futures::Future;

use self::rocksdb::DB;


pub fn inf_loop(db: db::DB, common_init: types::CommonInit)
{
    let mut io = IoHandler::new();
    io.add_method("terminate", |_: Params| {
        subprocess::exit(1);
    });
    /*
    io.add_method("payment", |_: Params| {
        
    });
*/

    
    
    loop {
        


        
    }
    
    
    
}

