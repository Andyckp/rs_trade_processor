use crossbeam::channel;
use rand::Rng;
use std::thread;
use std::time::Duration;

// Trade structure
#[derive(Debug, Copy, Clone)]
struct Trade {
    instrument_id: u64,
    price: f64,
    net_quantity: f64,
}

// TradeEvent (mutable placeholder for memory reuse)
struct TradeEvent {
    trade: Trade,
}

impl TradeEvent {
    fn update(&mut self, instrument_id: u64, price: f64, net_quantity: f64) {
        self.trade.instrument_id = instrument_id;
        self.trade.price = price;
        self.trade.net_quantity = net_quantity;
    }
}

// Trade processor (polls the channel continuously)
fn trade_processor(receiver: channel::Receiver<TradeEvent>) {
    while let Ok(trade_event) = receiver.recv() {
        println!("Processing trade: {:?}", trade_event.trade);
    }
}

// Trade publisher (generates random trades)
fn trade_publisher(sender: channel::Sender<TradeEvent>) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    

    for _ in 0..10 {
        let mut trade_event = TradeEvent { trade: Trade { instrument_id: 0, price: 0.0, net_quantity: 0.0 } };
        trade_event.update(
            rng.gen_range(1000..2000),
            rng.gen_range(50.0..500.0),
            rng.gen_range(1.0..100.0),
        );

        sender.send(trade_event).unwrap();
      
        thread::sleep(Duration::from_millis(1)); // Submit trade every 1ms
    }
}

// Test case: Runs publisher for 10s
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_processing() {
        let (tx, rx) = channel::bounded(1024); // Bounded channel with size 1024

        // Spawn trade processor thread
        let processor_thread = thread::spawn(move || trade_processor(rx));

        // Spawn trade publisher thread
        let publisher_thread = thread::spawn(move || trade_publisher(tx));

        // Wait for threads to complete
        publisher_thread.join().unwrap();
        processor_thread.join().unwrap();
    }
}