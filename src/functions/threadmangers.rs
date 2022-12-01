pub(crate) fn time_function() {
    let mut _check: u128 = 0;
    unsafe {
        UNSAFE_PUB_VAR.threads_on += 1.0;
        loop {
            _check = UNSAFE_PUB_VAR.amount_sent;
            thread::sleep(time::Duration::from_secs(1));
            UNSAFE_PUB_VAR.time = UNSAFE_PUB_VAR.amount_sent - _check;
        }
    }
}

pub(crate) fn add_start(mut val: MutexGuard<'static, SafeGlobalVar>) {
    val.thread_on += 1.0;
    unsafe {
        UNSAFE_PUB_VAR.threads_on += 1.0;
    }
}

