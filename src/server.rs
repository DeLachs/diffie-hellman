#[path = "dh_json_codec.rs"] mod dh_json_codec;
#[path = "dh_helpers.rs"] mod dh_helpers;

use std::io;
use std::net::{TcpListener, TcpStream};
use log::{debug, info, error};
use std::process::exit;
use num::BigInt;

use dh_json_codec::{Message, DHJsonCodec};
use dh_helpers::{calculate_gsp, calculate_key};

/// The server needs a IP Address with a port e.g. ``127.0.0.1:34612``.
pub fn server(addr: &str, p: BigInt, g: BigInt, secret: BigInt) -> io::Result<()> {
    let gsp_to_send = calculate_gsp(&g, &secret, &p);   // I could have done this outside this function.

    info!("Starting server on: {}", addr);

    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // The variables need to be cloned to ensure that the reference lives long enough.
            // Only needs to be done if the if statement is true.
            let p = p.clone();
            let g = g.clone();
            let secret = secret.clone();
            let gsp_to_send = gsp_to_send.clone();
            std::thread::spawn(move || {
                // The values don't need to be a reference because they are a one time use thing
                // that gets cloned before the new thread.
                handle_connection(stream, p, g, secret, gsp_to_send).map_err(|e| error!("{}", e))
            });
        }
    }
    Ok(())
}

/// Given a TcpStream:
/// - Deserialize the message
/// - Serialize and write the echo message to the stream
fn handle_connection(
    stream: TcpStream,
    p: BigInt,
    g: BigInt,
    secret: BigInt,
    gsp_to_send: BigInt
) -> io::Result<()> {
    let mut codec = DHJsonCodec::new(stream)?;
    // Read & Reverse the received message
    let message: Message = codec.read_message()?;
    match message {
        // TODO: documentation
        Message::HelloServer => {
            debug!("Received a HelloServer, sending: g, p and gsp_to_send");
            let answer = Message::NumbersServer {
                p: p.clone(),
                g: g.clone(),
                gsp: gsp_to_send.clone(),
            };
            codec.send_message(&answer)?;
        },
        Message::NumbersClient { gsp } => {
            //TODO: calculate key, print it and send OkServer for now
            let key = calculate_key(&gsp, &secret, &p);
            debug!("key: {}", key);
            codec.send_message(&Message::OkServer)?;
        }
        _ => {
            exit(1);
        }
    }

    // Read the real message
    //let message: String = codec.read_message()?;
    //info!("{}", message);
    Ok(())
}

