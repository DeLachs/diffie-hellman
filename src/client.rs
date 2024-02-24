#[path = "dh_json_codec.rs"] mod dh_json_codec;
#[path = "dh_helpers.rs"] mod dh_helpers;

use std::io;
use std::process::exit;
use std::net::TcpStream;
use dh_json_codec::{Message, DHJsonCodec};
use log::{debug, error};

use self::dh_helpers::{calculate_gsp, calculate_key, generate_secret};

pub fn send_stream(server: &str, _message: &str) -> io::Result<String> {
    // Establish a TCP connection with the far end
    let stream = TcpStream::connect(server)?;

    // Codec is our interface for reading/writing messages.
    // No need to handle reading/writing directly
    let mut codec = DHJsonCodec::new(stream)?;

    // Create and send the first message (``HelloServer`` (Client -> Server))
    codec.send_message(&Message::HelloServer)?;

    //? stupid test:
    // read answer
    let answer = codec.read_message()?;
    // open new stream to send new stuff. Needs to be done after receiving the answer.
    let stream = TcpStream::connect(server)?;
    let mut codec = DHJsonCodec::new(stream)?;
    match answer {
        Message::NumbersServer { g, p, gsp } => {
            let secret = generate_secret(&p);   //TODO decide where to generate secrets and do is on in roughly the same place on the server side.
            let gsp_to_send = calculate_gsp(&g, &secret, &p);
            // send gdp_to_send
            codec.send_message(&Message::NumbersClient { gsp: gsp_to_send })?;
            //TODO: print key for now
            let key = calculate_key(&gsp, &secret, &p);
            debug!("key: {}", key);
        }
        _ => {
            error!("Received an invalid response from the server: {}", server);
            exit(1);
        }
    }
    // read answer
    let answer = codec.read_message()?;
    // open new stream to send new stuff. Needs to be done after receiving the answer.
    let stream = TcpStream::connect(server)?;
    let mut _codec = DHJsonCodec::new(stream)?;
    match answer {
        Message::OkServer => {
            debug!("Received OkServer");
        }
        _ => {
            error!("Received an invalid response from the server: {}", server);
            exit(1);
        }
    }

    Ok("test".to_string())
}
