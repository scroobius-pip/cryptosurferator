use crate::lib::Operand::*;

pub enum ConstantOperator {
    PortfolioValue,
    CurrentMarketPrice, //operand is the index of market
}

pub type ConstantOperation = (ConstantOperator, Operand);
