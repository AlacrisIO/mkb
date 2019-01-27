use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use types;

//mod types;


pub fn read_common_init_from_file<P: AsRef<Path>>(path: P) -> Result<types::CommonInit, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `CommonInit`.
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}
