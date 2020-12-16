use std::error::Error;

use std::str;

use tokio::net::UdpSocket;
use std::net::SocketAddr;

use futures::stream::StreamExt;

const ADDRESS: &str = "0.0.0.0:6000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("Welcome to Tokio");

  let socket = UdpSocket::bind(ADDRESS.parse::<SocketAddr>().unwrap()).await?;

  // Feed service
  let mut buf = [0; 1024];
  loop {
    tokio::select! {
      Ok((len, addr)) = socket.recv_from(&mut buf) => {
        println!("{:?} bytes received from {:?}", len, addr);
      }
    }
  }
}
