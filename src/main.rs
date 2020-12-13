use std::error::Error;

//use gpio_cdev::{Chip, LineRequestFlags};

//use std::{thread, time};
use std::str;

use tokio::net::UdpSocket;
use std::net::SocketAddr;

/*
const ON: u8 = 1;
const OFF: u8 = 0;

const PIN_SWITCH:u32 = 26;
const PIN_MOTOR:u32 = 4;
const PIN_IR:u32 = 16;
*/

const ADDRESS: &str = "0.0.0.0:6000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("Welcome to Tokio");

  let socket = UdpSocket::bind(ADDRESS.parse::<SocketAddr>().unwrap()).await?;

  let mut buf = [0; 1024];
  loop {
    tokio::select! {
      r = socket.recv_from(&mut buf) => {
        let (len, addr) = r.unwrap();
        println!("{:?} bytes received from {:?}", len, addr);
      }
    }
  }
}

/*
  let mut chip = Chip::new("/dev/gpiochip0")?;
  let switch = chip
    .get_line(PIN_SWITCH)?
    .request(LineRequestFlags::INPUT, 0, "switch")?;
  println!("Value: {:?}", switch.get_value()?);

  let motor = chip
    .get_line(PIN_MOTOR)?
    .request(LineRequestFlags::OUTPUT, 0, "motor")?;

  let ir = chip
    .get_line(PIN_IR)?
    .request(LineRequestFlags::INPUT, 0, "ir")?;


  loop {
    let mut buf = [0; 8];
    let (amt, sender) = socket.recv_from(&mut buf)?;

    let msg = str::from_utf8(&buf).unwrap();
    println!("Recieved: {}\nlen:{}\tfrom:{:?}",
         msg, amt, sender);
    //motor.set_value(switch.get_value()?)?;
    
    println!("Dispense...\n");
    while ir.get_value()? == ON {
      println!("waiting for ir clear...\n");
      thread::sleep(poll_delay);
    }
    while switch.get_value()? == ON {
      motor.set_value(ON)?;
      thread::sleep(poll_delay);
    }

    println!("Wait for debounce");
    thread::sleep(debounce_delay);

    println!("now until switch breaks...\n");
    while switch.get_value()? == OFF {
      motor.set_value(ON)?;
      thread::sleep(poll_delay);
    }

    motor.set_value(OFF)?;
    println!("Done!\n");
  }
*/
