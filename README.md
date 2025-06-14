# rs_trade_processor contains data, data structures and business logics (stubs) for a trade processor using Rust
# Although it does not contain concrete logics, it contains major architectural composable components to build the application.
# The next tasks include 
# (1) add Aeron and SBE support
# (2) evaluate shared memory usage between components as different processes
# (3) evaluate disruptor vs custom built ringbuffer in terms of functionalities and performance
# (4) implemenmt padding for data structure to minimise false sharing