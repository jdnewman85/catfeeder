use std::error::Error;

use std::str;

use tokio::net::UdpSocket;
use std::net::SocketAddr;

use futures::stream::StreamExt;

use gpio_cdev::{Chip, AsyncLineEventHandle, LineRequestFlags, EventRequestFlags};

const PIN_IR:u32 = 16;
const PIN_SWITCH:u32 = 26;

/*
const ON: u8 = 1;
const OFF: u8 = 0;
const PIN_MOTOR:u32 = 4;
*/

const ADDRESS: &str = "0.0.0.0:6000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("Cat feeder");

  let socket = UdpSocket::bind(ADDRESS.parse::<SocketAddr>().unwrap()).await?;

  // Feed service
  let mut chip = Chip::new("/dev/gpiochip0")?;
  let switch_line = chip.get_line(PIN_SWITCH)?;
  let mut switch_stream = AsyncLineEventHandle::new(
    switch_line.events(
      LineRequestFlags::INPUT,
      EventRequestFlags::BOTH_EDGES,
      "switch"
    )?
  )?;
  let ir_line = chip.get_line(PIN_IR)?;
  let mut ir_stream = AsyncLineEventHandle::new(
    ir_line.events(
      LineRequestFlags::INPUT,
      EventRequestFlags::BOTH_EDGES,
      "ir"
    )?
  )?;

  let mut buf = [0; 1024];
  loop {
    tokio::select! {
      Ok((len, addr)) = socket.recv_from(&mut buf) => {
        println!("{:?} bytes received from {:?}", len, addr);
      }

      Some(event) = switch_stream.next() => {
        println!("{:?} event", event);
      }

      Some(event) = ir_stream.next() => {
        println!("{:?} event", event);
      }
    }
  }
}
