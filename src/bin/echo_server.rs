#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use core::fmt;
use std::io;
use std::error::Error;
use std::net::SocketAddr;

use clap::Parser;
use echo_client_server::{Args, BUFFER_SIZE};
use error_stack::{Result, IntoReport, ResultExt, Report};

use log::{error, info};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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

// TODO: Should I create a simple unit type `BindError` to return here instead
//   of `io::Error`? `BindError` would (to me) read better here, but it would
//   require all that boilerplate and I'm not sure that the minimal help here
//   would justify the fuss.
// We could create an enum type like `ConnectionError` that includes things
// like parsing errors and binding errors.
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

fn log_communication_error(err: &Report<ServerError>) {
    error!("\n{err:?}");
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    env_logger::init();

    let args = Args::parse();
    let server_address = SocketAddr::new(args.ip_address, args.port);
    info!("The server is listening at {server_address}");

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
    // There shouldn't be an error unwrapping the peer address since the connection
    // has already been established.
    #[allow(clippy::unwrap_used)]
    let peer_ip = socket.peer_addr().unwrap().ip();
    info!("Handling a stream from IP address {peer_ip:?}");

    let mut buf = [0; BUFFER_SIZE];
    loop {
        let num_read_bytes 
            = socket
                .read(&mut buf)
                .await
                .report()
                .change_context(SocketCommunicationError::Read)?;
        if num_read_bytes == 0 {
            info!("Done handling stream from IP address {peer_ip:?}");
            return Ok(());
            // The following lines force a communication error which might be useful
            // for testing.
            // return Err(Report::new(SocketCommunicationError::Read)
            //             .attach_printable("We forced an error when the socket closed."));
        }
        socket
            .write_all(&buf[0..num_read_bytes])
            .await
            .report()
            .change_context(SocketCommunicationError::Write)?;
    }
}
