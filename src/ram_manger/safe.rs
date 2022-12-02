use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct SafeGlobalVar {
    pub thread_on: f64,
    pub threads_allowed: f64,
}

pub static SAFE_PUB_VAR: Lazy<Mutex<SafeGlobalVar>> = Lazy::new(|| {
    Mutex::new(SafeGlobalVar {
        thread_on: 0.0,
        threads_allowed: 0.0,
    })
});
