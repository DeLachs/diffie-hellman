#[path = "lines_codec.rs"] mod lines_codec;

use std::io;
use std::net::TcpStream;
use lines_codec::LinesCodec;

pub fn send_stream(server: &str, message: &str) -> io::Result<String> {
    // Establish a TCP connection with the far end
    let stream = TcpStream::connect(server)?;

    // Codec is our interface for reading/writing messages.
    // No need to handle reading/writing directly
    let mut codec = LinesCodec::new(stream)?;

    // Serializing & Sending is nor just one line
    codec.send_message(message)?;

    let received_message = codec.read_message()?;

    Ok(received_message)
}
