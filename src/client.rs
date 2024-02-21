#[path = "lines_codec.rs"] mod lines_codec;

use std::io;
use std::process::exit;
use std::net::TcpStream;
use lines_codec::LinesCodec;
use log::error;

//TODO: working with a struct to store these things goes in the right direction
// I could do it on the server the same way but with a list to store the structs of multiple servers...
// or a hashmap to map these to there IP.
/*
struct Client {
    p: BigInt,
    g: BigInt,
    secret_number: BigInt,
    key: BigInt,
}
*/

pub fn send_stream(server: &str, message: &str) -> io::Result<String> {
    // Establish a TCP connection with the far end
    let stream = TcpStream::connect(server)?;

    // Codec is our interface for reading/writing messages.
    // No need to handle reading/writing directly
    let mut codec = LinesCodec::new(stream)?;

    // Serializing & Sending is nor just one line
    codec.send_message("HELLOSERVER")?;

    let received_message = codec.read_message()?;
    
    if received_message == "OK" {
        codec.send_message(message)?;
    }
    else {
        error!("received no OK from the server: {}", server);
        exit(1);
    }

    let received_message = codec.read_message()?;

    Ok(received_message)
}
