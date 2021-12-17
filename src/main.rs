mod lib;
use barter::data::handler::historic::{HistoricCandleHandler, HistoricDataLego};
use barter::data::handler::{Continuation, Continuer, MarketGenerator};
use barter::data::market::MarketEvent;
use barter::strategy::strategy::{Config as StrategyConfig, RSIStrategy};
use chrono::{DateTime, TimeZone, Utc};
use std::fs::File;
use CSVCandleIterator::CSVCandleData;
// use barter_data::model::Candle;
use barter_data::model::{Candle, MarketData};
use lib::{operation::trade::TradeList, operation::*};

fn main() {
    // let operations: OperationList = vec![];
    // let mut trade_list: TradeList = vec![];
    // operations[operations.len() - 1].evaluate(&operations, &mut trade_list);

    // //print all items in  TradeList vector
    // for trade in trade_list {
    //     println!("{}", trade);
    // }
    // let lego = HistoricDataLego {
    //     exchange: "Binance",
    //     symbol: "btcusdt".to_string(),
    //     candle_iterator: vec![Candle::default()].into_iter(),
    // };

    // let mut data = HistoricCandleHandler::new(lego);
    // loop {
    //     let market_event = match data.can_continue() {
    //         Continuation::Continue => match data.generate_market() {
    //             Some(market_event) => market_event,
    //             None => continue,
    //         },
    //         Continuation::Stop => {
    //             break;
    //         }
    //     };

    //     match market_event.data {
    //         MarketData::Candle(candle) => {
    //             println!("{}", candle.close);
    //         }
    //         MarketData::Trade(trade) => {
    //             println!("{}", trade.price);
    //         }
    //     }
    // }
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open("src/data/1inch.csv").expect("file not found"));

    let candle_iterator = rdr.deserialize().map(|result| {
        let (
                open_time,
                open,
                high,
                low,
                close,
                volume,
                close_time,
                _,
                number_of_trades,
                _,
                _,
                _,
            ): CSVCandleData = result.expect("error");

        Candle {
            close,
            high,
            low,
            open,
            start_timestamp: Utc.timestamp_millis(open_time as i64),
            end_timestamp: Utc.timestamp_millis(close_time as i64),
            volume: volume as f64,
            trade_count: number_of_trades,
        }
    });
}

mod CSVCandleIterator {
    use barter_data::model::Candle;
    type Timestamp = u64;
    type Opentime = Timestamp;
    type Closetime = Timestamp;
    type Open = f64;
    type High = f64;
    type Low = f64;
    type Close = f64;
    type Volume = f64;
    type TradeCount = u64;
    type TakerBuyBaseAssetVolume = f64;
    type QuoteAssetVolume = f64;
    type TakerBuyQuoteAssetVolume = f64;
    type Ignore = u32;

    pub type CSVCandleData = (
        Opentime,
        Open,
        High,
        Low,
        Close,
        Volume,
        Closetime,
        QuoteAssetVolume,
        TradeCount,
        TakerBuyBaseAssetVolume,
        TakerBuyQuoteAssetVolume,
        Ignore,
    );
    // type CSVCandleIterator = std::vec::IntoIter<CSVCandleData>;

    struct CSVCandleIterator {
        candle: Option<Candle>,
    }
    impl CSVCandleIterator {
        fn new(&self, file: std::fs::File) -> Self {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);
            for result in rdr.deserialize() {
                // let record: CSVCandleData = result?;
                let record: CSVCandleData = result.unwrap();
                println!("{:?}", record);
            }

            Self { candle: None }
        }
    }
}
