use std::sync::{atomic::Ordering, Arc};
use std::thread;
use std::time::Duration;
use rand::Rng;

use crate::data_structure::{EnrichedTradeRingBuffer, InstrumentRingBuffer, TradeRingBuffer, BUFFER_SIZE};

// TradeProcessor is the transport-and-codec-agnostic inner-business-logic component
// TODO add transport and codec components which are external to this module  
pub struct TradeProcessor {
    pub trade_buffer: Arc<TradeRingBuffer>,
    pub instrument_buffer: Arc<InstrumentRingBuffer>,
    pub enriched_buffer: Arc<EnrichedTradeRingBuffer>,
}

impl TradeProcessor {
    pub fn new() -> Self {
        Self {
            trade_buffer: TradeRingBuffer::new(),
            instrument_buffer: InstrumentRingBuffer::new(),
            enriched_buffer: EnrichedTradeRingBuffer::new(),
        }
    }

    pub fn process(&self) {
    loop {
        while let Some(instrument) = self.instrument_buffer.read() {
            println!("Processed InstrumentEvent: {:?}", instrument);
        }

        while let Some(trade) = self.trade_buffer.read() {
            let portfolio_id = ['X'; 16]; // TODO produce portfolio due to instrument details

            loop {
                let write_pos = self.enriched_buffer.get_write_pos();
                let read_pos = self.enriched_buffer.get_read_pos();

                if write_pos - read_pos < BUFFER_SIZE {
                    let slot = write_pos % BUFFER_SIZE;
                    self.enriched_buffer.write_enriched_trade(
                        slot,
                        trade.id,
                        trade.instrument_id,
                        trade.price,
                        trade.net_quantity,
                        portfolio_id,
                        trade.instrument_id,
                    );
                    self.enriched_buffer.inc_write_pos();
                    println!("Processed TradeEvent: {:?}", trade);
                    break;
                } else {
                    thread::sleep(Duration::from_millis(1)); // Wait for space in buffer
                }
            }
        }
            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn start(self) -> thread::JoinHandle<()> {
        thread::spawn(move || self.process())
    }
}

// Original: Test case for trade processing
#[test]
fn given_trade_processor_when_receive_raw_trade_and_instrument_then_process_enriched_trade() {
    let processor = TradeProcessor::new();
    let trade_buffer = processor.trade_buffer.clone();
    let instrument_buffer = processor.instrument_buffer.clone();
    let enriched_buffer = processor.enriched_buffer.clone();
    processor.start(); 

    let trade_count = 1000;
    let instrument_count = 500;
    let timeout = Duration::from_secs(100);
    let start_time = std::time::Instant::now();

    thread::spawn(move || test_trade_publisher(trade_buffer.clone(), trade_count));
    thread::spawn(move || test_instrument_publisher(instrument_buffer.clone(), instrument_count));

    let mut processed_trades = 0;
    while start_time.elapsed() < timeout && processed_trades < trade_count {
        if enriched_buffer.read().is_some() {
            processed_trades += 1;
        }
        thread::sleep(Duration::from_millis(1));
    }

    assert_eq!(processed_trades, trade_count);
}

fn test_trade_publisher(trade_buffer: Arc<TradeRingBuffer>, trade_count: usize) {
    let mut rng = rand::thread_rng();
    for i in 1..=trade_count {
        loop {
            let write_pos = trade_buffer.get_write_pos();
            let read_pos = trade_buffer.get_read_pos();

            if write_pos - read_pos < BUFFER_SIZE {
                let slot = write_pos % BUFFER_SIZE;
                trade_buffer.write_trade(
                    slot,
                    i as u64,
                    (i % 10) as u64,
                    rng.gen_range(100.0..200.0),
                    rng.gen_range(-50.0..50.0),
                );
                trade_buffer.inc_write_pos();
                println!("Published TradeEvent: id={}", i);
                break;
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}

fn test_instrument_publisher(instrument_buffer: Arc<InstrumentRingBuffer>, instrument_count: usize) {
    let mut rng = rand::thread_rng();
    for i in 0..instrument_count {
        loop {
            let write_pos = instrument_buffer.get_write_pos();
            let read_pos = instrument_buffer.get_read_pos();

            if write_pos - read_pos < BUFFER_SIZE {
                let slot = write_pos % BUFFER_SIZE;
                instrument_buffer.write_instrument(
                    slot,
                    i as u64,
                    if rng.gen_bool(0.5) { 'C' } else { 'P' },
                    rng.gen_range(50.0..250.0),
                    rng.gen_range(1700000000..1800000000),
                );
                instrument_buffer.inc_write_pos();
                println!("Published InstrumentEvent: id={}", i);
                break;
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}