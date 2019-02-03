//use std::process;
//use std::error::Error;
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
//use log::{info};

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
pub fn read_common_init_ci<P: AsRef<Path>>(path: P) -> types::CommonInit {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("Error in opening path");
    println!("read_common_init_ci : After open statement");
    
    let reader = BufReader::new(file);
    println!("read_common_init_ci : After reader statement");
    

    // Read the JSON contents of the file as an instance of `CommonInit`.
    let u : types::CommonInit = serde_json::from_reader(reader).expect("Error in parsing of input");
    println!("read_common_init_ci : We have read u");
    u
}



pub fn read_local_init<P: AsRef<Path>>(path: P) -> types::LocalInit {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("Error read_local_init, operation");
    let reader = BufReader::new(file);

    let u : types::LocalInit = serde_json::from_reader(reader).expect("Error reading types::LocalInit");
    u
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
            info!("read_t_json reached an exception");
            process::exit(1);
        },
    }
}
*/
