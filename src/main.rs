mod benchmark;
mod prime_number;
mod primitive_root;
mod dh_helpers;
mod dh_json_codec;

use std::process::exit;
use std::env;
use log::{debug, info, error};
use env_logger::Env;
use std::io;
use std::net::{TcpListener, TcpStream};
use num::BigInt;

use prime_number::generate_prime_number;
use primitive_root::generate_primitive_root;
use dh_json_codec::{Message, DHJsonCodec};
use dh_helpers::{calculate_gsp, calculate_key, generate_secret};

struct Args {
    is_server: bool,
    addr: String,
}

fn main() {
    // Initialize logging with env_logger and the default level of info.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    debug!("{:?}", args);
    let args = Args {
        is_server: if args[1] == "-s".to_string() {true} else {false},
        addr: {
            if args.len() == 3 {
                args[2].clone()
            } else {
                "127.0.0.1:34612".to_string()
            }
        }
    };

    //? SERVER
    if args.is_server {
        let result = server(args.addr.as_str());
        match result {
            Ok(()) => (),
            Err(e) => {
                error!("{}", e);
                exit(1);
            }
        };
    }
    
    //? CLIENT
    if !args.is_server {
        let result = time_function!(send_stream(args.addr.as_str()));
        match result {
            Ok(()) => (),
            Err(e) => {
                error!("{}", e);
                exit(1);
            }
        };
    }
}

//? SERVER
/// The server needs a IP Address with a port e.g. ``127.0.0.1:34612``.
fn server(addr: &str) -> io::Result<()> {
    info!("Generating `p` and `g`");
    // Generating the ``p``, ``g`` and the secret before starting to listen.
    // 4096 long number needs to be a prime number.
    let p = generate_prime_number(4096);
    let g = generate_primitive_root(&p);
    // Code below not working and I don't know why.
    //let p = time_function!(generate_prime_number(4096));
    //let g = time_function!(generate_primitive_root(&p));
    let secret = generate_secret(&p);
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
        // If client send Message::HelloServer -> respond with Message::NumbersServer.
        Message::HelloServer => {
            debug!("Received a HelloServer, sending: g, p and gsp_to_send");
            let answer = Message::NumbersServer {
                p: p.clone(),
                g: g.clone(),
                gsp: gsp_to_send.clone(),
            };
            codec.send_message(&answer)?;
        },
        // If client send Message::NumbersClient -> calculate key and respond with Message::OkServer.
        Message::NumbersClient { gsp } => {
            let key = calculate_key(&gsp, &secret, &p);
            info!("key: {}", key);
            codec.send_message(&Message::OkServer)?;
        }
        _ => {
            error!("Client send an invalid message!");
            exit(1);
        }
    }
    Ok(())
}

//? CLIENT
fn send_stream(server: &str) -> io::Result<()> {
    // Establish a TCP connection with the far end.
    let stream = TcpStream::connect(server)?;
    // Codec is our interface for reading/writing messages.
    // No need to handle reading/writing directly.
    let mut codec = DHJsonCodec::new(stream)?;

    // Send first message to Server -> Message::HelloServer.
    codec.send_message(&Message::HelloServer)?;

    // Read first answer from server.
    let answer = codec.read_message()?;
    // Open new stream to send new stuff. Needs to be done after receiving the answer.
    let stream = TcpStream::connect(server)?;
    let mut codec = DHJsonCodec::new(stream)?;
    match answer {
        Message::NumbersServer { g, p, gsp } => {
            // Use the number from the server to generate the secret number to use in the DHKE.
            let secret = generate_secret(&p);
            let gsp_to_send = calculate_gsp(&g, &secret, &p);
            // Send the server the number ``gdp_to_send``.
            codec.send_message(&Message::NumbersClient { gsp: gsp_to_send })?;
            // and calculate the key.
            let key = calculate_key(&gsp, &secret, &p);
            info!("key: {}", key);
        }
        _ => {
            error!("Received an invalid response from the server: {}", server);
            exit(1);
        }
    }
    // Read second answer from the server.
    let answer = codec.read_message()?;
    // Don't open a new stream here because there is nothing to send.
    match answer {
        Message::OkServer => {
            info!("Received OkServer");
        }
        _ => {
            error!("Received an invalid response from the server: {}", server);
            exit(1);
        }
    }

    Ok(())
}
