use std::sync::{Mutex, MutexGuard, PoisonError};

use once_cell::sync::Lazy;
use reqwest::RequestBuilder;

pub struct SafeGlobalVar {
    pub thread_on: f64,
    pub threads_allowed: f64,
}

impl SafeGlobalVar {
    pub fn get() -> Option<MutexGuard<'static, SafeGlobalVar>> {
        let mut error: u8 = 0;
        loop {
            match SAFE_PUB_VAR.lock() {
                Ok(data) => { return Some(data); }
                Err(_) => {
                    error += 1;
                    if error > 128 {
                        return None;
                    }
                }
            };
        }
    }
}

pub static SAFE_PUB_VAR: Lazy<Mutex<SafeGlobalVar>> = Lazy::new(|| {
    Mutex::new(SafeGlobalVar {
        thread_on: 0.0,
        threads_allowed: 0.0,
    })
});

pub struct UnsafePubVar {
    pub attack_mode: bool,
    pub attack_url: String,
    pub amount_sent: u128,
    pub time: u128,
    pub threads_on: f64,
    pub http_sender: RequestBuilder,
    pub headers: Vec<String>,
    pub headers_val: Vec<String>,
    pub proxy: Vec<RequestBuilder>,
}

pub static mut UNSAFE_PUB_VAR: Lazy<UnsafePubVar> = Lazy::new(|| UnsafePubVar {
    time: 0,
    attack_url: "".to_owned(),
    amount_sent: 0,
    threads_on: 0.0,
    http_sender: reqwest::Client::new().get(""),
    headers: vec![],
    headers_val: vec![],
    attack_mode: false,
    proxy: vec![],
});
