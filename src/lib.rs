use std::net::IpAddr;

use clap::Parser;

pub const BUFFER_SIZE: usize = 128;
pub const DEFAULT_PORT: u16 = 7707;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
   /// The IP address to connect to
   #[clap(short, long, value_parser, default_value = "127.0.0.1")]
   pub ip_address: IpAddr,

   /// The port number to connect to
   #[clap(short, long, value_parser, default_value_t = DEFAULT_PORT)]
   pub port: u16,
}
