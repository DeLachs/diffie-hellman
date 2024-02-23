use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::exit;

pub struct LinesCodec {
    // Our buffered reader & writer
    reader: io::BufReader<TcpStream>,
    writer: io::BufWriter<TcpStream>,
}

impl LinesCodec {
    /// Encapsulate a TcpStream with buffered reader/writer functionality
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        // Both BufReader and LineWriter need to own a stream
        // We can clone the stream to simulate splitting Tx & Rx with ``try_clone()``
        let writer = io::BufWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self {reader, writer})
    }

    /// Write the given message to the TcpStream
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        println!("message length: {}", message.len());
        let length = message.len();
        let length = length.to_be_bytes();
        self.writer.write(&length)?;
        self.writer.write(&message.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }

    /// Read a received message from the TcpStream
    pub fn read_message(&mut self) -> io::Result<String> {
        let mut length = [0; 8];
        self.reader.read_exact(&mut length)?;
        let length = usize::from_be_bytes(length);  //TODO: u64 not usize
        println!("message length: {}", length);

        let mut test_vec: Vec<u8> = vec![0; length];
        self.reader.read_exact(&mut test_vec)?;
        let test = String::from_utf8(test_vec);
        let test = match test {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        };
        println!("{}", test);
        //TODO return json
        Ok(test)
    }
}

