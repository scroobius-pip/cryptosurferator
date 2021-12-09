use crate::lib::operand::*;
//market data intervals
#[derive(Copy, Clone)]
pub enum MarketDataInterval {
    Minute1,
    Minute3,
    Minute5,
    Minute15,
    Minute30,
    Hour1,
    Hour2,
    Hour4,
    Hour6,
    Hour8,
    Hour12,
    Day1,
    Day3,
    Week1,
    Month1,
}

//binary constant operators
pub enum MarketDataOperator {
    Volume,
    TradeCount,
    Open,
    High,
    Low,
    Close,
    OrderBookBids,
    OrderBookAsks,
}
type MarketIndex = Operand;
type MarketDataIndexStart = Operand;
type MarketDataIndexStop = Operand;

pub type MarketDataOperation = (
    MarketDataOperator,
    MarketIndex,
    MarketDataIndexStart,
    MarketDataIndexStop,
    MarketDataInterval,
);

pub fn get_market_data(
    market_index: usize,
    data_index_start: usize,
    data_index_stop: usize,
    market_interval: MarketDataInterval,
) -> MarketData {
    let market_data = MarketData {
        open: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        high: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        low: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        close: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        volume: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        asks: vec![],
        bids: vec![],
        trade_count: vec![],
    };
    market_data
}

pub struct MarketData {
    pub open: Vec<f32>,
    pub high: Vec<f32>,
    pub low: Vec<f32>,
    pub close: Vec<f32>,
    pub volume: Vec<f32>,
    pub trade_count: Vec<f32>,
    pub bids: Vec<f32>,
    pub asks: Vec<f32>,
}
