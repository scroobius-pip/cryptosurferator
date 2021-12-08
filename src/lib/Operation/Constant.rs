use crate::lib::Operand::*;

pub enum ConstantOperator {
    PortfolioValue,
    MarketPrice, //operand is the index of market
    
}

pub type ConstantOperation = (ConstantOperator, Operand);
