use std::process;
//xuse std::error::Error;


//extern crate rocksdb;
use rocksdb::DB;


use log::{info};


pub fn open_database(file_database: &String) -> DB {
//    let db : Result<DB,Error> = DB::open_default(file_database);
    let db = DB::open_default(file_database);
    match db {
        Ok(v) => v,
        Err(_) => {
            info!("Error reading the database file_satabase={}", file_database);
            process::exit(1);
        },
    }
}

