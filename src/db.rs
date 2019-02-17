use std::process;
//xuse std::error::Error;


//extern crate rocksdb;
use rocksdb::DB;
use types::*;

//use log::{info};

pub struct DBE {
    pub db: DB,
    pub iter: i32,
}



pub fn open_database(file_database: &String) -> DBE {
    let db = DB::open_default(file_database);
    match db {
        Ok(v) => DBE {db: v, iter: 0},
        Err(_) => {
            println!("Error reading the database file_satabase={}", file_database);
            process::exit(1);
        },
    }
}

pub fn database_update(mut w: std::sync::MutexGuard<DBE>, esumreq: SumTypeRequest) {
    let iter = (*w).iter;
    let iter_str = iter.to_string();
    let value_str : String = serde_json::to_string(&esumreq).unwrap();
    //
    let key_u8_b = iter_str.as_bytes();
    let value_u8_b = value_str.as_bytes();
    (*w).db.put(key_u8_b, value_u8_b).unwrap();
    let iter_inc : i32 = iter + 1;
    (*w).iter = iter_inc;
}
