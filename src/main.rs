use std::error::Error;

use std::str;

use std::net::SocketAddr;
use tokio::net::UdpSocket;

use bytes::BytesMut;
use tokio_util::codec::Decoder;
use tokio_util::udp::UdpFramed;

use futures::stream::StreamExt;

use bytes::Buf;
use tokio::sync::mpsc;

use gpio_cdev::{AsyncLineEventHandle, Chip, EventRequestFlags, LineRequestFlags};

use tokio::time::{sleep, Duration};

const PIN_IR: u32 = 16;
const PIN_SWITCH: u32 = 26;

const MOTOR_ON: u8 = 1;
const MOTOR_OFF: u8 = 0;
const PIN_MOTOR: u32 = 4;

const IR_ON: u8 = 1;
//const IR_OFF: u8 = 0;

const SWITCH_ON: u8 = 1;
const SWITCH_OFF: u8 = 0;

const DELAY_DEBOUNCE: u64 = 200;

#[derive(Debug)]
struct FeedPacket {
    data: String,
}
#[derive(Debug)]
struct Packet;

impl Decoder for Packet {
    type Item = FeedPacket;
    type Error = std::io::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let remaining = src.remaining();
        let source_bytes = src.copy_to_bytes(remaining);
        src.clear();

        if remaining > 0 {
            let s = String::from_utf8(source_bytes.to_vec()).unwrap();
            Ok(Some(FeedPacket { data: s }))
        } else {
            Ok(None)
        }
    }
}

const ADDRESS: &str = "0.0.0.0:7000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Cat feeder");

    let socket = UdpSocket::bind(ADDRESS.parse::<SocketAddr>().unwrap()).await?;
    let mut framed_socket = UdpFramed::new(socket, Packet {});

    let (tx, mut rx) = mpsc::unbounded_channel();

    // Feed service
    let mut chip = Chip::new("/dev/gpiochip0")?;

    let ir_line = chip.get_line(PIN_IR)?;

    let switch_line = chip.get_line(PIN_SWITCH)?;

    let motor = chip
        .get_line(PIN_MOTOR)?
        .request(LineRequestFlags::OUTPUT, 0, "motor")?;

    tokio::spawn(async move {
        while let Some(feed_packet) = rx.recv().await {
            dbg!(&feed_packet);
            // Wait for ir off
            let ir_state = ir_line
                .request(LineRequestFlags::INPUT, 0, "ir")
                .unwrap()
                .get_value()
                .unwrap();
            if ir_state == IR_ON {
                let mut ir_stream = AsyncLineEventHandle::new(
                    ir_line
                        .events(
                            LineRequestFlags::INPUT,
                            EventRequestFlags::FALLING_EDGE,
                            "ir",
                        )
                        .unwrap(),
                )
                .unwrap();
                ir_stream.next().await;
            }

            // Motor on
            motor.set_value(MOTOR_ON).expect("Unable to turn motor on!");

            // Wait for switch off
            let switch_state = switch_line
                .request(LineRequestFlags::INPUT, 0, "switch")
                .unwrap()
                .get_value()
                .unwrap();
            if switch_state == SWITCH_ON {
                let mut switch_stream = AsyncLineEventHandle::new(
                    switch_line
                        .events(
                            LineRequestFlags::INPUT,
                            EventRequestFlags::FALLING_EDGE,
                            "switch",
                        )
                        .unwrap(),
                )
                .unwrap();
                switch_stream.next().await;
            }

            // Wait for debounce
            sleep(Duration::from_millis(DELAY_DEBOUNCE)).await;

            // Wait for switch on
            let switch_state = switch_line
                .request(LineRequestFlags::INPUT, 0, "switch")
                .unwrap()
                .get_value()
                .unwrap();
            if switch_state == SWITCH_OFF {
                let mut switch_stream = AsyncLineEventHandle::new(
                    switch_line
                        .events(
                            LineRequestFlags::INPUT,
                            EventRequestFlags::RISING_EDGE,
                            "switch",
                        )
                        .unwrap(),
                )
                .unwrap();
                switch_stream.next().await;
            }

            //Motor off
            motor
                .set_value(MOTOR_OFF)
                .expect("Unable to turn motor off!");
        }
    });

    while let Some(Ok(feed_packet)) = framed_socket.next().await {
        dbg!(&feed_packet);
        tx.send(feed_packet).unwrap();
    }

    Ok(())
}
