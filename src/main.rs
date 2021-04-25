use std::error::Error;

use std::str;

use tokio::net::UdpSocket;
use std::net::SocketAddr;

use bytes::{BytesMut};
use tokio_util::codec::Decoder;
use tokio_util::udp::UdpFramed;

use futures::stream::StreamExt;

/*
use gpio_cdev::{Chip, AsyncLineEventHandle, LineRequestFlags, EventRequestFlags};

const PIN_IR:u32 = 16;
const PIN_SWITCH:u32 = 26;

const ON: u8 = 1;
const OFF: u8 = 0;
const PIN_MOTOR:u32 = 4;
*/

#[derive(Debug)]
struct FeedPacket {
    data: String
}
#[derive(Debug)]
struct Packet;

impl Decoder for Packet {
    type Item = FeedPacket;
    type Error = std::io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let s = str::from_utf8(&src).unwrap();
        /*
        let s = match str::from_utf8(&data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8: {}", e),
        };
        */
        Ok(Some(FeedPacket{
            data: String::from(s)
        }))
    }
}
const ADDRESS: &str = "0.0.0.0:6000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("Cat feeder");

  let socket = UdpSocket::bind(ADDRESS.parse::<SocketAddr>().unwrap()).await?;
  let mut framed_socket = UdpFramed::new(socket, Packet{});

  /*
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
  */

  while let Some(Ok(feed_packet)) = framed_socket.next().await {
      dbg!("FeedPacket: {:?}", feed_packet);
  }

  Ok(())
  /*
  loop {
    tokio::select! {
      /*
      Ok((len, addr)) = socket.recv_from(&mut buf) => {
        println!("{:?} bytes received from {:?}", len, addr);
      }
      */
      Some(_feed_packet) = framed_socket.next() => {
        println!("Packet recieved!");
      }

      Some(event) = switch_stream.next() => {
        println!("{:?} event", event);
      }

      Some(event) = ir_stream.next() => {
        println!("{:?} event", event);
      }
    }
  }
*/
}
