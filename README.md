# rs_trade_processor contains data, data structures and business logics (stubs) for a trade processor using Rust
Although it does not contain concrete logics, it contains major architectural composable components to build the application.
The main purpose of the project is to learn Rust's capability and specifically the low level functionalities including
(1) struct-based data oriented programming
(2) multi-threading and its related message exchange facilities, e.g. MPMC ringbuffer and duty loop, no matter partitioning by functional unit or by data key
(3) efficient transport (multicast, shared memory, etc...) and binary codec
(4) explore the macro compiler time based programming to leverage static dispatch

# The next tasks include 
(1) add Aeron and SBE support
(2) evaluate shared memory usage between components as different processes
(3) evaluate disruptor vs custom built ringbuffer in terms of functionalities and performance
(4) implement cache-line padding for data structure to minimise false sharing