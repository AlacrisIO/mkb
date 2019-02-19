use std::net::TcpStream;
//use std::str;
use std::io::{self, BufRead, BufReader, Write};


use types::*;

pub fn send_message(mesg: Message) -> Result<String,io::Error> {
    let mut stream = TcpStream::connect(mesg.ip_plus_port)
        .expect("Could not connect to server");

    stream.write(mesg.message.as_bytes())
        .expect("Failed to write to server");

    let mut reader = BufReader::new(&stream);

    let mut buffer: Vec<u8> = Vec::new();
    reader.read_until(b'\n', &mut buffer)
        .expect("Could not read into buffer");
    let str : String = String::from_utf8(buffer).unwrap();
    Ok(str)
}

