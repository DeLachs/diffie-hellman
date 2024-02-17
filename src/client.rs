use std::io::{self, Write};
use std::net::TcpStream;

pub fn send_stream() -> io::Result<()> {
    // Establish a TCP connection with the far end
    let mut stream = TcpStream::connect("127.0.0.1:34612")?;


    let data = b"Hello";    
    // ``write_all()`` will return ``Err(io::Error(io::ErrorKind::Interrupted))``
    // if it is unable to queue all bytes.
    stream.write_all(data)?;
    // Tell TCP to send the buffered data on the wire
    stream.flush()?;

    Ok(())
}

/*
let mut stream = TcpStream::connect("127.0.0.1:34612")?;

is equal to:

let mut stream = TcpStream::connect("127.0.0.1:34612")?;
let mut stream = match stream {
    Ok(c) => c,
    Err(e) => return Err(e)
};
*/