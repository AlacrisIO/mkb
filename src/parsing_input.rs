use std::process;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use types;

//mod types;


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
            println!("read_common_init_ci reached an exception");
            process::exit(1);
        },
    }

    
}

