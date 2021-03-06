//amount of leverage for futures trading, x1 for normal trading
use crate::lib::op::operand::*;
use std::fmt::Display;
type MarketIndex = Operand;
type MarketPrice = Operand;
type MarketAmount = Operand;

#[derive(Copy, Clone, Debug, PartialEq)]
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
    // TradeLeverage,
);

#[derive(Copy, Clone, Debug)]
pub enum TradeOperator {
    Buy,
    Sell,
    Nothing,
}

impl PartialEq for TradeOperator {
    fn eq(&self, other: &TradeOperator) -> bool {
        match (self, other) {
            (TradeOperator::Buy, TradeOperator::Buy) => true,
            (TradeOperator::Sell, TradeOperator::Sell) => true,
            (TradeOperator::Nothing, TradeOperator::Nothing) => true,
            _ => false,
        }
    }
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

#[derive(Debug, PartialEq)]
pub struct Trade {
    pub operator: TradeOperator,
    pub index: usize,
    pub price: f32,
    pub amount: f32,
    // pub leverage: TradeLeverage,
}

//implment Display for Trade struct
impl Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Trade: {}, {}, {}, {}",
            self.operator, self.index, self.price, self.amount,
        )
    }
}

#[test]
fn test_partial_eq() {
    let trade1 = Trade {
        operator: TradeOperator::Buy,
        index: 1,
        price: 1.0,
        amount: 1.0,
        // leverage: TradeLeverage::X1,
    };
    let trade2 = Trade {
        operator: TradeOperator::Buy,
        index: 1,
        price: 1.0,
        amount: 1.0,
        // leverage: TradeLeverage::X1,
    };
    assert_eq!(trade1, trade2);

    let trade3 = Trade {
        operator: TradeOperator::Sell,
        index: 1,
        price: 1.0,
        amount: 1.0,
        // leverage: TradeLeverage::X1,
    };

    assert_ne!(trade1, trade3);
    assert_eq!(trade1, trade1);
}
