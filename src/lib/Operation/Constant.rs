use crate::lib::operand::*;

pub enum ConstantOperator {
    PortfolioValue, //Total value of all assets in usdt
    MarketPrice,    //operand is the index of market
    SelectedMarketIndex,
    ///this is relative to the value the market portfolio had at the time when the position was opened,
    /// it can be negative if there was a loss and positive if there was a gain
    SelectedMarketPortfolioRelativeValue,
    SelectedMarketPortfolioValue,
    BtcMarketIndex,
    EthMarketIndex, 
    // SentimentValue //sentiment score of an asset
}

pub type ConstantOperation = (ConstantOperator, Operand);
