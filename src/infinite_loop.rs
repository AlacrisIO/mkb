use std::process;
use jsonrpc_core::*;
use jsonrpc_core::futures::Future;

use rocksdb::DB;

use types;

pub fn inf_loop(db: DB, common_init: types::CommonInit)
{
    let mut io = IoHandler::new();
    io.add_method("terminate", |_: Params| {
        Ok(Value::String("We are trying to exit from the terminate".into()))
    });
    io.add_method("add_account", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("deposit", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("payment", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("withdrawal", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("send_data", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    io.add_method("get_data", |_: Params| {
        Ok(Value::String("adding account to the syste,".into()))
    });
    
    loop {
        


        
    }
    
    
    
}

