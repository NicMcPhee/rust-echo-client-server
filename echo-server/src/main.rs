#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{io::{Error, Read, Write}, net::{TcpListener, TcpStream}};

const BUFFER_SIZE: usize = 128;

// Issues to deal with:
// - Our error handling is fairly weak.
// - We have no threading, so we only handle one connection at a time.
// - There should be "proper" logging added to this.

fn main() -> Result<(), Error> {
    // TODO: Let the user set this port via the command line.
    let listener = TcpListener::bind("127.0.0.1:60606")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        // TODO: We might want to log or print something here
        // if we get an Error type in `stream`.
        // TODO: We also should handle any error coming back from
        // `echo_stream`.
        echo_stream(stream?)?;
    }
    Ok(())
}

fn echo_stream(mut stream: TcpStream) -> Result<(), Error> {
    println!("Handling a stream: {:?}", stream);
    let mut buf = [0; BUFFER_SIZE];
    // TODO: Handle the error case here.
    loop {
        let num_read_bytes = stream.read(&mut buf)?;
        if num_read_bytes == 0 {
            println!("Done handling stream {:?}", stream);
            return Ok(());
        }
        // TODO: Handle the error case here.
        stream.write_all(&buf[0..num_read_bytes])?;
    }
}
