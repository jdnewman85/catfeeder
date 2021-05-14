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

const IO_CHIP: &str = "/dev/gpiochip0";

const PIN_IR: u32 = 16;
const PIN_SWITCH: u32 = 26;
const PIN_MOTOR: u32 = 4;

const MOTOR_ON: u8 = 1;
const MOTOR_OFF: u8 = 0;

const IO_ON: u8 = 1;
const IO_OFF: u8 = 0;

const DELAY_DEBOUNCE: u64 = 200; //ms

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
            let data = String::from_utf8(source_bytes.to_vec()).unwrap();
            Ok(Some(FeedPacket{data}))
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
    let mut chip = Chip::new(IO_CHIP)?;

    let ir_line = chip.get_line(PIN_IR)?;

    let switch_line = chip.get_line(PIN_SWITCH)?;

    let motor = chip
        .get_line(PIN_MOTOR)?
        .request(LineRequestFlags::OUTPUT, 0, "motor")?;

    tokio::spawn(async move {
        while let Some(feed_packet) = rx.recv().await {
            dbg!(&feed_packet);
            wait_for_line(&ir_line, IO_OFF).await;
            motor.set_value(MOTOR_ON).expect("Unable to turn motor on!");
            wait_for_line(&switch_line, IO_OFF).await;
            sleep(Duration::from_millis(DELAY_DEBOUNCE)).await;
            wait_for_line(&switch_line, IO_ON).await;
            motor.set_value(MOTOR_OFF).expect("Unable to turn motor off!");
        }
    });

    while let Some(Ok(feed_packet)) = framed_socket.next().await {
        dbg!(&feed_packet);
        tx.send(feed_packet).unwrap();
    }

    Ok(())
}

fn io_state_to_edge_flag(state: u8) -> gpio_cdev::EventRequestFlags {
    match state {
        IO_ON => EventRequestFlags::RISING_EDGE,
        IO_OFF => EventRequestFlags::FALLING_EDGE,
        _ => panic!("Unhandled IO state: {}", state)
    }
}

async fn wait_for_line(line: &gpio_cdev::Line, target_state: u8) {
    // Wait for line
    let state = line
        .request(LineRequestFlags::INPUT, 0, "ir").unwrap()
        .get_value().unwrap();
    if state != target_state {
        let target_edge = io_state_to_edge_flag(state);
        let mut ir_stream = AsyncLineEventHandle::new(
            line.events(
                LineRequestFlags::INPUT,
                target_edge,
                "ir",
            ).unwrap(),
        ).unwrap();
        ir_stream.next().await;
    }
}
