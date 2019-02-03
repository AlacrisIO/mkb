use std::process;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
//use serde::{Serialize, Serializer, Deserialize, Deserializer};

//use serde::Deserializer;
//use serde::de::DeserializeOwned;
//use serde::Deserialize;
use serde_json;
//use serde::de::Deserialize;
//use serde::ser::Serialize;

use types;
use log::{info};

//mod types;




/// Returns the common initialization file or an error.
///
/// # Arguments
///
/// * `identifier` - The path where the data is located.
///
/// # Remarks
///
/// The input file is in JSON format.
pub fn read_common_init_ci_exn<P: AsRef<Path>>(path: P) -> Result<types::CommonInit, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `CommonInit`.
    let u : types::CommonInit = serde_json::from_reader(reader)?;
    Ok(u)
}





pub fn read_common_init_ci<P: AsRef<Path>>(path: P) -> types::CommonInit {
    let u_res : Result<types::CommonInit, Box<Error>> = read_common_init_ci_exn(path);
    match u_res {
        Ok(v) => v,
        Err(_) => {
            info!("read_common_init_ci reached an exception");
            process::exit(1);
        },
    }
}



pub fn read_local_init_exn<P: AsRef<Path>>(path: P) -> Result<types::LocalInit, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `CommonInit`.
    let u : types::LocalInit = serde_json::from_reader(reader)?;
    Ok(u)
}


pub fn read_local_init<P: AsRef<Path>>(path: P) -> types::LocalInit {    
    let u_res : Result<types::LocalInit, Box<Error>> = read_local_init_exn(path);
    match u_res {
        Ok(v) => v,
        Err(_) => {
            info!("read_common_init_ci reached an exception");
            process::exit(1);
        },
    }
}

/*
pub fn read_t_json_exn<T: Deserialize>(path: String) -> Result<T, Box<Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u : T = serde_json::from_reader(reader)?;
    Ok(u)
}
*/

/*
pub fn read_t_json<P: AsRef<Path>, T>(path: P) -> T {
    let u_res : Result<T, Box<Error>> = read_t_json_exn(path);
    match u_res {
        Ok(v) => v,
        Err(_) => {
            info!("read_common_init_ci reached an exception");
            process::exit(1);
        },
    }
}
*/
