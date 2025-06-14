use aeron_rs::context::Context;

use aeron_rs::aeron::Aeron;
use aeron_rs::subscription::Subscription;
use std::ffi::CString;
use std::thread::sleep;


const STREAM_ID: i32 = 1001;
const CHANNEL: &str = "aeron:ipc";  // Shared memory transport

#[cfg(test)]
#[test]
fn test_aeron_client() {
    let mut context = Context::new();
    context.set_aeron_dir("/tmp/aeron1".to_string());
    // context.set_term_buffer_length(1048576);
    let mut aeron = Aeron::new(context).expect("Failed to initialize Aeron");

    let channel = CString::new("aeron::ipc").expect("Failed to create CString");

    let subscription = aeron.add_subscription(channel, STREAM_ID)
        .expect("Failed to subscribe to IPC channel");
}
