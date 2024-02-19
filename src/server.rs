#[path = "lines_codec.rs"] mod lines_codec;

use std::io;
use std::net::{TcpListener, TcpStream};
use log::{info, error};

use lines_codec::LinesCodec;

/// The server needs a IP Address with a port e.g. ``127.0.0.1:34612``.
pub fn server(addr: &str) -> io::Result<()> {
    info!("Starting server on: {}", addr);

    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                handle_connection(stream).map_err(|e| error!("{}", e))
            });
        }
    }
    Ok(())
}


/// Given a TcpStream:
/// - Deserialize the message
/// - Serialize and write the echo message to the stream
fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let mut codec = LinesCodec::new(stream)?;
    // Read & Reverse the received message
    let message: String = codec
        .read_message()
        // Reverse message
        .map(|m| m.chars().rev().collect())?;

    info!("{}", message);

    // And use the codec to return it
    codec.send_message(&message)?;
    Ok(())
}

