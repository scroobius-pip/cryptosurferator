//amount of leverage for futures trading, x1 for normal trading
// use crate::lib::Operand
use std::fmt::Display;
type MarketIndex = Operand;
type MarketPrice = Operand;
type MarketAmount = Operand;

#[derive(Copy, Clone)]
pub enum TradeLeverage {
    X1,
    X2,
    X5,
}

impl Display for TradeLeverage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TradeLeverage::X1 => write!(f, "X1"),
            TradeLeverage::X2 => write!(f, "X2"),
            TradeLeverage::X5 => write!(f, "X5"),
        }
    }
}

pub type TradeOperation = (
    TradeOperator,
    MarketIndex,
    MarketPrice,
    MarketAmount,
    TradeLeverage,
);

#[derive(Copy, Clone)]
pub enum TradeOperator {
    Buy,
    Sell,
    Nothing,
}
//implement Display trait for TradeOperator
impl Display for TradeOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TradeOperator::Buy => write!(f, "Buy"),
            TradeOperator::Sell => write!(f, "Sell"),
            TradeOperator::Nothing => write!(f, "Nothing"),
        }
    }
}

pub type TradeList = Vec<Trade>;

pub struct Trade {
    operator: TradeOperator,
    index: usize,
    price: f32,
    amount: f32,
    leverage: TradeLeverage,
}

//implment Display for Trade struct
impl Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Trade: {}, {}, {}, {}, {}",
            self.operator, self.index, self.price, self.amount, self.leverage
        )
    }
}
