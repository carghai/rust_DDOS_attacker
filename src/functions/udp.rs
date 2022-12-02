use std::{thread, time};
use std::net::UdpSocket;

pub(crate) fn udp() -> UdpSocket {
    let mut error_much: u8 = 0;
    loop {
        match UdpSocket::bind("0.0.0.0:8080") {
            Ok(data) => {
                return data;
            }
            Err(data) => {
                if error_much > 10 {
                    panic!(
                        "Failed when starting udp, please check 8080 port and try again\n {}",
                        data
                    );
                }
                thread::sleep(time::Duration::from_millis(20));
                error_much += 1;
            }
        }
    }
}
