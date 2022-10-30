use std::io;

use crate::extra_fn::proxy_set;
use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};

pub struct AttackData {
    pub ai_mode: bool,
    pub udp_mode: bool,
}

pub fn where_attack() -> AttackData {
    let mut return_data = AttackData {
        ai_mode: false,
        udp_mode: false,
    };
    println!("Where to attack?(give url)");
    unsafe {
        UNSAFE_PUB_VAR.attack_url = get_input().trim().to_owned();
    }
    println!("Do you want to use UDP(y or n)");
    return_data.udp_mode = true_or_no();
    if !return_data.udp_mode {
        println!("Ai Mode?(y/n), This is helpful if you have a fast pc and/or you bottle neck is your wifi!");
        return_data.ai_mode = true_or_no();
        println!("Headers?(supports headers like headerKey1|headerVal1,headerKey2|headerVal2,headerKey3|headerVal3 you don't need to fill out and if you don't want this press n");
        loop {
            let headers_unparsed = get_input().trim().to_owned();
            match &*headers_unparsed {
                "n" => {
                    break;
                }
                _ => {
                    let parsing = headers_unparsed.split(',');
                    let check_amount = parsing.clone().count();
                    let mut amount_looped: u8 = 0;
                    for unwrapped in parsing {
                        let mut final_unwrap = unwrapped.split('|');
                        match final_unwrap.nth_back(0) {
                            None => {
                                syntax_error();
                            }
                            Some(data) => unsafe {
                                UNSAFE_PUB_VAR.headers.push(data.to_owned());
                                match final_unwrap.next() {
                                    None => {
                                        syntax_error();
                                    }
                                    Some(second_data) => {
                                        UNSAFE_PUB_VAR.headers_val.push(second_data.to_owned());
                                        amount_looped += 1;
                                    }
                                }
                            }
                        }
                    }
                    if check_amount == amount_looped as usize {
                        break;
                    }
                }
            };
        }
        loop {
            println!("Proxy? (if you don't want one hit n)");
            let unparsed_str = get_input().trim().to_owned();
            match &*unparsed_str {
                "n" => {
                    println!("{}", proxy_set(vec![], false).expect("Failed when setting http client"));
                    break;
                }
                _ => {
                    let loop_num = unparsed_str.split(',');
                    for proxy in loop_num {
                        let error = proxy_set(vec![proxy.trim()], true);
                        match error {
                            Err(e) => println!("{}", e),
                            Ok(yay) => {
                                println!("{}", yay);
                                break;
                            }
                        }
                    }
                }
            }
        }
    } else {
        return_data.ai_mode = false;
    }
    loop {
        println!("Threads? (if you get a dns error lower threads)");
        let unparsed_str: Result<u64, std::num::ParseIntError> = get_input().trim().parse();
        match unparsed_str {
            Ok(num) => {
                if let Ok(mut data) = SAFE_PUB_VAR.lock() {
                    data.threads_allowed = num as f64;
                    break;
                } else {
                    println!("waiting on file lock");
                }
            }
            Err(e) => {
                println!("please write proper number\n (advanced error details: {})", e);
            }
        }
    }
    return_data
}


fn true_or_no() -> bool {
    loop {
        match get_input().trim() {
            "y" => {
                return true;
            }
            "n" => {
                return false;
            }
            _ => {
                println!("please say y or n");
            }
        }
    }
}

fn get_input() -> String {
    let mut val = "".to_owned();
    loop {
        let error = io::stdin()
            .read_line(&mut val);
        if error.is_ok() {
            return val;
        } else {
            println!("please try again");
        }
    }
}

fn syntax_error() {
    println!("please make it like this: headerKey1|headerVal1,headerKey2|headerVal2,headerKey3|headerVal3 and try again");
    unsafe {
        UNSAFE_PUB_VAR.headers = vec![];
        UNSAFE_PUB_VAR.headers_val = vec![];
    }
}