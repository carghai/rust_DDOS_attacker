extern crate core;

mod normal_attack;
mod ram_manger;
mod where_attack;
pub mod functions;
mod ai_attack;

#[tokio::main]
async fn main() {
    let init_data = where_attack::where_attack();
    if init_data.ai_mode {
        ai_attack::start().await;
    } else {
        normal_attack::start(init_data).await;
    }
}

// 2000 to can take down a replit flask api
// This is the code
// app.route('/hard')
// def hard_dos():
//   return "failure"
