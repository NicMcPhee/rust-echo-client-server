#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::io;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BUFFER_SIZE: usize = 128;

// Issues to deal with:
// - Our error handling is fairly weak.
// - There should be "proper" logging added to this.
// - Add something like `clap` to handle command line arguments
//   - We could specify the port number that way.

#[tokio::main]
async fn main() -> io::Result<()> {
    // TODO: Let the user set this port via the command line.
    // TODO: Handle errors when binding ot the address.
    let listener = TcpListener::bind("127.0.0.1:60606").await?;

    loop {
        // TODO: Handle errors when accepting requests.
        let (socket, _) = listener.accept().await?;
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
