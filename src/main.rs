mod lib;
use barter::data::handler::{
    historical::{HistoricalCandleHandler, HistoricalDataLego},
    Continuation, Continuer, MarketGenerator,
};
use chrono::{TimeZone, Utc};
use std::fs::File;
use csv_candle_iterator::CSVCandleData;
use barter_data::model::{Candle, MarketData};

fn main() {

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
            ): CSVCandleData = result.unwrap_or_default();

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

    let lego = HistoricalDataLego {
        exchange: "Binance",
        symbol: "1inchbtc".to_string(),
        candles: candle_iterator,
    };

    let mut data = HistoricalCandleHandler::new(lego);
    
    loop {
        let market_event = match data.can_continue() {
            Continuation::Continue => match data.generate_market() {
                Some(market_event) => market_event,
                None => continue,
            },
            Continuation::Stop => {
                break;
            }
        };

        match market_event.data {
            MarketData::Candle(candle) => {
                println!("{}", candle.close);
            }
            MarketData::Trade(trade) => {
                println!("{}", trade.price);
            }
        }
    }
}

mod csv_candle_iterator {
    
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
}

// pub struct TestHistoricDataLego<T: Iterator<Item = Candle>> {
//     pub exchange: &'static str,
//     pub symbol: String,
//     pub candle_iterator: T,
// }
