use std::{io::{self, Read, Write}, net::{IpAddr, SocketAddr}};

use clap::Parser;

use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

const BUFFER_SIZE: usize = 128;

/// Simple echo client
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// The IP address to connect to
   #[clap(short, long, value_parser, default_value = "127.0.0.1")]
   ip_address: IpAddr,

   /// The port number to connect to
   #[clap(short, long, value_parser, default_value_t = 60606)]
   port: u16,
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let server_address = SocketAddr::new(args.ip_address, args.port);
    let mut socket = TcpStream::connect(server_address).await?;
    let mut buf = [0; BUFFER_SIZE];

    loop {
        let stdio_bytes_read = io::stdin().read(&mut buf)?;
        if stdio_bytes_read == 0 {
            return Ok(());
        }
        socket.write_all(&buf[0..stdio_bytes_read]).await?;

        let socket_bytes_read = socket.read(&mut buf).await?;
        // Since we wrote out more than 0 bytes, we should have gotten
        // more than 0 bytes back in return.
        assert!(socket_bytes_read > 0);
        io::stdout().write_all(&buf[0..socket_bytes_read])?;
    }
}
