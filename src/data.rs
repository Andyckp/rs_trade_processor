

#[derive(Debug, Copy, Clone, Default)]
pub struct TradeEvent {
    pub id: u64,
    pub instrument_id: u64,
    pub price: f64,
    pub net_quantity: f64,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct InstrumentEvent {
    pub id: u64,
    pub instrument_type: char, // 'C' for Call, 'P' for Put
    pub strike_price: f64,
    pub expiry: u64,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct EnrichedTradeEvent {
    pub id: u64,
    pub instrument_id: u64,
    pub price: f64,
    pub net_quantity: f64,
    pub portfolio_id: [char; 16], 
    pub processed_timestamp: u64,
}
