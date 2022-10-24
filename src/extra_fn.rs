use std::{thread, time};
use std::net::UdpSocket;
use std::sync::MutexGuard;

use reqwest::{Error, Response};

use crate::ram_manger::{SafeGlobalVar, UNSAFE_PUB_VAR};

pub(crate) fn time_function() {
    unsafe {
        UNSAFE_PUB_VAR.threads_on += 1.0;
        loop {
            UNSAFE_PUB_VAR.amount_sent = 0.0;
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}


pub(crate) fn proxy_set(url: &str, proxy: bool) -> Result<String, Error> {
    if proxy {
        let proxy_set = reqwest::Proxy::all(url);
        match proxy_set {
            Err(e) => Err(e),
            Ok(good) => {
                let final_check = reqwest::Client::builder()
                    .proxy(good)
                    .build();
                match final_check {
                    Err(e) => Err(e),
                    Ok(final_data) => unsafe {
                        UNSAFE_PUB_VAR.http_sender = final_data;
                        Ok("Proxy has been set!".to_owned())
                    }
                }
            }
        }
    } else {
        unsafe {
            UNSAFE_PUB_VAR.http_sender = reqwest::Client::new();
        }
        Ok("Set http client with no proxy successfully!".to_owned())
    }
}

pub(crate) async fn request() -> Result<Response, Error> {
    unsafe {
        UNSAFE_PUB_VAR.http_sender
            .get(&UNSAFE_PUB_VAR.attack_url)
            .header(UNSAFE_PUB_VAR.headers.get(0)
                        .unwrap_or(&"".to_owned()),
                    UNSAFE_PUB_VAR
                        .headers_val
                        .get(0)
                        .unwrap_or(&"".to_owned()))
            .header(UNSAFE_PUB_VAR.headers.get(1)
                        .unwrap_or(&"".to_owned()),
                    UNSAFE_PUB_VAR
                        .headers_val
                        .get(1)
                        .unwrap_or(&"".to_owned()))
            .header(UNSAFE_PUB_VAR.headers.get(2)
                        .unwrap_or(&"".to_owned()),
                    UNSAFE_PUB_VAR
                        .headers_val
                        .get(2)
                        .unwrap_or(&"".to_owned()))
            .send()
            .await
    }
}

pub(crate) fn add_start(mut val: MutexGuard<'static, SafeGlobalVar>) {
    val.thread_on += 1.0;
    unsafe {
        UNSAFE_PUB_VAR.threads_on += 1.0;
    }
}

pub(crate) fn udp() -> UdpSocket {
    let mut error_much: u8 = 0;
    loop {
        match UdpSocket::bind("0.0.0.0:8080") {
            Ok(data) => { return data; }
            Err(data) => {
                if error_much > 10 {
                    panic!("Failed when starting udp, please check 8080 port and try again\n {}", data);
                }
                thread::sleep(time::Duration::from_millis(20));
                error_much += 1;
            }
        }
    }
}



