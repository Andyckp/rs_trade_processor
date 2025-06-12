use aeron_rs::context::Context;
use rs_trade_processor::start_server; // Use the function from lib.rs

// fn main() {
//     start_server();
// }



use aeron_rs::aeron::Aeron;
use aeron_rs::subscription::Subscription;
use std::ffi::CString;
use std::thread::sleep;
// use bincode;
// use Trade;

const STREAM_ID: i32 = 1001;
const CHANNEL: &str = "aeron:ipc";  // Shared memory transport

fn main() {
    let mut context = Context::new();
//    context.set_aeron_dir(CString::new("/tmp/aeron").unwrap());
    context.set_aeron_dir("/tmp/aeron1".to_string());
    // context.set_term_buffer_length(1048576);
    let mut aeron = Aeron::new(context).expect("Failed to initialize Aeron");
   
    let channel = CString::new("aeron::ipc").expect("Failed to create CString");
   
    let subscription = aeron.add_subscription(channel, STREAM_ID)
        .expect("Failed to subscribe to IPC channel");

    // loop {
    //     if let Ok(packet) = subscription.poll() {
    //         if let Ok(decoded_trade) = bincode::deserialize::<Trade>(&packet) {
    //             println!("Processed trade: {:?}", decoded_trade);
    //         }
    //     }
    //     sleep(Duration::from_millis(100));
    // }
}
