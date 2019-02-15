use std::process;
//xuse std::error::Error;


//extern crate rocksdb;
use rocksdb::DB;


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

