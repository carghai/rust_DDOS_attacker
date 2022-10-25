use std::{thread, time};
use std::net::UdpSocket;
use std::sync::MutexGuard;

use reqwest::{Error, RequestBuilder, Response};

use crate::ram_manger::{SafeGlobalVar, UNSAFE_PUB_VAR};

pub(crate) fn time_function() {
    let time = 0;
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
                        UNSAFE_PUB_VAR.http_sender = request_builder(final_data);
                        Ok("Proxy has been set!".to_owned())
                    }
                }
            }
        }
    } else {
        unsafe {
            UNSAFE_PUB_VAR.http_sender = request_builder(reqwest::Client::new());
        }
        Ok("Set http client with no proxy successfully!".to_owned())
    }
}

pub(crate) async fn request() -> Result<Response, Error> {
    unsafe {
        let request = UNSAFE_PUB_VAR.http_sender.try_clone();
        match request {
            Some(sender) => sender.send().await,
            None => {
                println!("Ram Error, Lower Threads");
                reqwest::Client::new().get(&UNSAFE_PUB_VAR.attack_url).send().await
            }
        }
    }
}

pub(crate) fn request_builder(client: reqwest::Client) -> RequestBuilder {
    unsafe {
        let mut https_builder = client
            .get(&UNSAFE_PUB_VAR.attack_url);
        for (index, header) in UNSAFE_PUB_VAR.headers.iter().enumerate() {
            let use_header = UNSAFE_PUB_VAR.headers_val.get(index);
            match use_header {
                None => {
                    println!("Header Val Data Was Damaged, please restart the client but don't worry it is skipping this header")
                }
                Some(data) => {
                    https_builder = https_builder.header(header, data)
                }
            }
        }
        https_builder
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



