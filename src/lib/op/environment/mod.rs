use super::operation::market_data::MarketData;

// Operations can call the environment to get information about the outside world
pub trait Env {
    fn get_market_index_list(&self) -> Vec<f32> {
        vec![1.0, 2.0, 3.0]
    }
    fn get_market_price(&self, index: usize) -> f32 {
        match index {
            0 => 1.0,
            1 => 2.0,
            2 => 3.0,
            _ => 0.0,
        }
    }

    fn get_market_portfolio_value(&self, index: usize) -> f32 {
        match index {
            0 => 1.0,
            1 => 2.0,
            2 => 3.0,
            _ => 0.0,
        }
    }

    fn get_overall_portfolio_value(&self) -> f32 {
        1.0
    }

    fn get_usdt_market_index(&self) -> usize {
        0
    }

    fn get_btc_market_index(&self) -> usize {
        1
    }

    fn get_eth_market_index(&self) -> usize {
        2
    }

    fn get_market_data(&self,market_index: usize, timestamp_start: f32, duration: f32) -> MarketData {
        let market_data = MarketData {
            open: vec![1.0, 2.0, 3.0, 4.0, 5.0]
                .into_iter()
                .map(|x| x * market_index as f32)
                .collect(),
            high: vec![1.0, 2.0, 3.0, 4.0, 5.0]
                .into_iter()
                .map(|x| x * (1 / (market_index + 1)) as f32)
                .collect(),
            low: vec![1.0, 2.0, 3.0, 4.0, 5.0]
                .into_iter()
                .map(|x| x * market_index as f32)
                .collect(),
            close: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            volume: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            trade_count: vec![],
        };
        market_data
    }
    
}

