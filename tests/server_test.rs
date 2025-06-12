#[cfg(test)]
mod test {
    use rs_trade_processor::start_server;

    #[test]
    fn test_server_startup() {
        start_server(); // Call function to ensure it runs
    }
}

