#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use core::fmt;
use std::io;
use std::error::Error;
use std::net::{SocketAddr, AddrParseError};

use error_stack::{Result, IntoReport, ResultExt, Report};

use log::{error, info};

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

#[derive(Debug)]
enum SocketCommunicationError {
    Read,
    Write
}

impl fmt::Display for SocketCommunicationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("there was an error while communicating on a socket")
    }
}

impl Error for SocketCommunicationError {}

fn parse_server_address(addr: &str) -> Result<SocketAddr, AddrParseError> {
    addr
        .parse::<SocketAddr>()
        .report()
        .attach_printable_lazy(|| {
            format!("Could not parse '{addr}' as a socket address")
        })
}

// TODO: Should I create a simple unit type `BindError` to return here instead
//   of `io::Error`? `BindError` would (to me) read better here, but it would
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

fn log_communication_error(err: &Report<ServerError>) {
    error!("{err:?}");
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    env_logger::init();
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
            let res = echo_stream(socket)
                .await
                .attach_printable_lazy(|| {
                    format!("Failure when communicating with socket at address {server_address}.")
                })
                .change_context(ServerError);
            if let Err(err) = res {
                log_communication_error(&err);
            }
        });

    }
}

async fn echo_stream(mut socket: TcpStream) -> Result<(), SocketCommunicationError> {
    info!("Handling a stream: {:?}", socket);
    let mut buf = [0; BUFFER_SIZE];
    // TODO: Handle the error case here.
    loop {
        let num_read_bytes 
            = socket
                .read(&mut buf)
                .await
                .report()
                .change_context(SocketCommunicationError::Read)?;
        if num_read_bytes == 0 {
            info!("Done handling stream {:?}", socket);
            return Ok(());
        }
        socket
            .write_all(&buf[0..num_read_bytes])
            .await
            .report()
            .change_context(SocketCommunicationError::Write)?;
    }
}
