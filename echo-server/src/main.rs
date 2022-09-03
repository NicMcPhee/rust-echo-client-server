#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use core::fmt;
use std::io;
use std::error::Error;
use std::net::{SocketAddr, AddrParseError};

use error_stack::{Result, IntoReport, ResultExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BUFFER_SIZE: usize = 128;

#[derive(Debug)]
struct ServerError;

impl fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("there was an error in the server")
    }
}

impl Error for ServerError {}

fn parse_server_address(addr: &str) -> Result<SocketAddr, AddrParseError> {
    addr
        .parse::<SocketAddr>()
        .report()
        .attach_printable_lazy(|| {
            format!("Could not parse '{addr}' as a socket address")
        })
}

// TODO: Should I create a simple unit type `BindError` to return here instead
//   of `io::Error`. `BindError` would (to me) read better here, but it would
//   require all that boilerplate and I'm not sure that the minimal help here
//   would justify the fuss.
async fn bind_to_address(addr: SocketAddr) -> Result<TcpListener, io::Error> {
    TcpListener::bind(addr)
        .await
        .report()
        .attach_printable_lazy(|| {
            format!("Could not attach to address {addr}.")
        })
}

async fn accept_connection(listener: &TcpListener) -> Result<TcpStream, io::Error> {
    listener
        .accept()
        .await
        .report()
        .map(|(socket, _)| socket)
}

// Issues to deal with:
// - Our error handling is fairly weak.
// - There should be "proper" logging added to this.
// - Add something like `clap` to handle command line arguments
//   - We could specify the port number that way.

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // TODO: Let the user set this port via the command line.
    let server_address = "127.0.0.1:60606";
    let server_address: SocketAddr 
        = parse_server_address(server_address)
            .change_context(ServerError)?;
    let listener 
        = bind_to_address(server_address)
            .await
            .change_context(ServerError)?;

    loop {
        let socket 
            = accept_connection(&listener)
                .await
                .change_context(ServerError)?;
        tokio::spawn(async move {
            // TODO: Handle error when processing socket.
            echo_stream(socket).await.expect("There was a problem handling a socket");
        });
    }
}

async fn echo_stream(mut socket: TcpStream) -> io::Result<()> {
    println!("Handling a stream: {:?}", socket);
    let mut buf = [0; BUFFER_SIZE];
    // TODO: Handle the error case here.
    loop {
        // Handle error when reading from socket.
        let num_read_bytes = socket.read(&mut buf).await?;
        if num_read_bytes == 0 {
            println!("Done handling stream {:?}", socket);
            return Ok(());
        }
        // TODO: Handle the error case here.
        socket.write_all(&buf[0..num_read_bytes]).await?;
    }
}
