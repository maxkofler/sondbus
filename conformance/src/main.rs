use std::{
    env::args,
    thread::sleep,
    time::{Duration, SystemTime},
};

use serial2::SerialPort;
use sondbus::crc8::{CRC8Autosar, CRC};

fn main() {
    let args: Vec<String> = args().collect();
    let port = args[1].clone();

    let port = SerialPort::open(port, 1_000_000).unwrap();

    sleep(Duration::from_secs(2));

    loop {
        let data = [0x55, 0x10, 0x0, 0x01, 0x00];
        let crc = CRC8Autosar::new().update_move(&data);
        port.write_all(&data).unwrap();
        port.write_all(&[crc.finalize()]).unwrap();
        port.flush().unwrap();

        let mut x = [0u8];
        if let Err(e) = port.read_exact(&mut x) {
            println!("{} => {}", humantime::format_rfc3339(SystemTime::now()), e);
            continue;
        };

        if x[0] != 0x55 {
            println!(
                "{} CRC ERROR => {:x}",
                humantime::format_rfc3339(SystemTime::now()),
                x[0]
            );
        }

        //sleep(Duration::from_millis(1));
    }
}
