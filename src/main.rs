extern crate core;

use crate::attacks::{ai, normal};

mod ram_var;
mod where_attack;
pub mod functions;
pub mod attacks;

#[tokio::main]
async fn main() {
    let init_data = where_attack::where_attack();
    if init_data.ai_mode {
        ai::start().await;
    } else {
        normal::start(init_data).await;
    }
}

// 2000 to can take down a replit flask api
// This is the code
// app.route('/hard')
// def hard_dos():
//   return "failure"
