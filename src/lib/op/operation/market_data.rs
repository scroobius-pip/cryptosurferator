use crate::lib::op::operand::*;

//binary constant operators
pub enum MarketDataOperator {
    Volume,
    TradeCount,
    Open,
    High,
    Low,
    Close,
    // OrderBookBids,
    // OrderBookAsks,
}

type MarketIndex = Operand;
type MarketDataTimestampStart = Operand;
type MarketDataDuration = Operand;

//the concept of market tickers is not needed,
// simply calculate the market data with dynamic durations and a start timestamp
pub type MarketDataOperation = (
    MarketDataOperator,
    MarketIndex,
    MarketDataTimestampStart,
    MarketDataDuration,
);

 
pub struct OrderBook {
    pub bid_price: Vec<f32>,
    pub bid_volume: Vec<f32>,
    pub ask_price: Vec<f32>,
    pub ask_volume: Vec<f32>,
}

pub struct MarketData {
    pub open: Vec<f32>,
    pub high: Vec<f32>,
    pub low: Vec<f32>,
    pub close: Vec<f32>,
    pub volume: Vec<f32>,
    pub trade_count: Vec<f32>,
}
