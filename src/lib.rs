use std::net::IpAddr;

use clap::Parser;

pub const BUFFER_SIZE: usize = 128;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
   /// The IP address to connect to
   #[clap(short, long, value_parser, default_value = "127.0.0.1")]
   pub ip_address: IpAddr,

   /// The port number to connect to
   #[clap(short, long, value_parser, default_value_t = 60606)]
   pub port: u16,
}
