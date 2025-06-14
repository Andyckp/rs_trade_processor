
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::ptr;
use crate::data::{TradeEvent, InstrumentEvent, EnrichedTradeEvent};

pub const BUFFER_SIZE: usize = 1024;

// Macro to generate RingBuffer implementations
macro_rules! impl_ring_buffer {
    ($buffer_name:ident, $struct_name:ident, $write_fn_name:ident, $($field_name:ident: $field_type:ty),*) => {
        pub struct $buffer_name {
            buffer: Box<[$struct_name; BUFFER_SIZE]>,
            read: AtomicUsize,
            write: AtomicUsize,
        }

        impl $buffer_name {
            pub fn new() -> Arc<Self> {
                Arc::new(Self {
                    buffer: Box::new([$struct_name::default(); BUFFER_SIZE]),
                    read: AtomicUsize::new(0),
                    write: AtomicUsize::new(0),
                })
            }

            pub fn $write_fn_name(&self, slot: usize, $($field_name: $field_type),*) {
                unsafe {
                    let item_ptr = self.buffer.as_ptr().add(slot) as *mut $struct_name;
                    $( (*item_ptr).$field_name = $field_name; )*
                }
            }

            pub fn inc_write_pos(&self) {
                self.write.fetch_add(1, Ordering::Release);
            }

            pub fn read(&self) -> Option<$struct_name> {
                let read_pos = self.read.load(Ordering::Acquire);
                let write_pos = self.write.load(Ordering::Acquire);

                if read_pos < write_pos {
                    unsafe {
                        let item_ptr = self.buffer.as_ptr().add(read_pos % BUFFER_SIZE);
                        let item = ptr::read(item_ptr);
                        self.read.fetch_add(1, Ordering::Release);
                        Some(item)
                    }
                } else {
                    None
                }
            }

            pub fn get_write_pos(&self) -> usize {
                self.write.load(Ordering::Acquire)
            }

            pub fn get_read_pos(&self) -> usize {
                self.read.load(Ordering::Acquire)
            }
        }
    };
}

impl_ring_buffer!(TradeRingBuffer, TradeEvent, write_trade, id: u64, instrument_id: u64, price: f64, net_quantity: f64);
impl_ring_buffer!(InstrumentRingBuffer, InstrumentEvent, write_instrument, id: u64, instrument_type: char, strike_price: f64, expiry: u64);
impl_ring_buffer!(EnrichedTradeRingBuffer, EnrichedTradeEvent, write_enriched_trade, id: u64, instrument_id: u64, price: f64, net_quantity: f64, portfolio_id: [char; 16], processed_timestamp: u64);
