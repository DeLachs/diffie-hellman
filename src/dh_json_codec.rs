use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::exit;
use log::{debug, error};
use num::BigInt;
use serde::{Deserialize, Serialize};

/// Constructs the messages that can be send from client and server.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum Message {
    HelloServer,
    NumbersServer {
        p: BigInt,
        g: BigInt,
        gsp: BigInt,
    },
    NumbersClient {
        gsp: BigInt,
    },
    OkServer,
}

pub struct DHJsonCodec {
    // Our buffered reader & writer
    reader: io::BufReader<TcpStream>,
    writer: io::BufWriter<TcpStream>,
}

impl DHJsonCodec {
    /// Encapsulate a TcpStream with buffered reader/writer functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        // Both BufReader and LineWriter need to own a stream
        // We can clone the stream to simulate splitting Tx & Rx with ``try_clone()``
        let writer = io::BufWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self {reader, writer})
    }

    /// Write the given message to the TcpStream
    pub fn send_message(&mut self, message: &Message) -> io::Result<()> {
        debug!("json send: {:?}", message);
        // Convert the Message to a json string
        let message = serde_json::to_string(message)?;
        // Check if message.len() is longer then 2 to the power of 16,
        // because that is the length limit introduced a few lines below.
        if message.len() > 65536 {
            error!("Message to long!");
            exit(1);
        }
        debug!("message send length: {}", message.len());
        let length: u16 = message.len().try_into().unwrap();
        let length: [u8; 2] = length.to_be_bytes();
        self.writer.write(&length)?;
        self.writer.write(&message.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }

    /// Read a received message from the TcpStream
    pub fn read_message(&mut self) -> io::Result<Message> {
        let mut length = [0; 2];
        self.reader.read_exact(&mut length)?;
        let length = u16::from_be_bytes(length);  //TODO: u64 not usize
        debug!("message receive length: {}", length);

        let mut test_vec: Vec<u8> = vec![0; usize::from(length)];
        self.reader.read_exact(&mut test_vec)?;
        let test = String::from_utf8(test_vec);
        let test = match test {
            Ok(x) => x,
            Err(e) => {
                error!("{}", e);
                exit(1);
            }
        };
        let j: Message = serde_json::from_str(&test)?;
        debug!("json received: {:?}", j);
        Ok(j)
    }
}
