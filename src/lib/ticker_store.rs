use lerp::Lerp;

struct TickerStore {
    tickers: Vec<Ticker>,
    ticker_size: u64, //in minutes
    start_timestamp: u64,
}

#[derive(Copy, Clone, Lerp)]
struct Ticker {
    high: f32,
    low: f32,
}

impl TickerStore {
    fn new(ticker_size: u64, start_timestamp: u64) -> TickerStore {
        TickerStore {
            tickers: Vec::new(),
            ticker_size,
            start_timestamp,
        }
    }

    fn get_ticker(&self, timestamp: u64) -> Ticker {
        let index = self.timestamp_to_index(timestamp);
        self.tickers[index]
    }

    fn get_ticker_lerp(&self, timestamp: u64) -> Ticker {
        let index = self.timestamp_to_float_index(timestamp);
        if index < 0.0 {
            Ticker {
                high: 0.0,
                low: 0.0,
            }
        } else if index > self.tickers.len() as f64 {
            self.tickers[self.tickers.len() - 1]
        } else {
            let prev_index = (index - 1 as f64) as usize;
            let next_index = (index + 1 as f64) as usize;
            let next_index = if next_index >= self.tickers.len() {
                self.tickers.len()
            } else {
                next_index
            };

            let prev_ticker = &self.tickers[prev_index];
            let next_ticker = &self.tickers[next_index];

            //lerp between prev and next
            let lerp_value = (index - prev_index as f64) / (next_index as f64 - prev_index as f64);
            let lerp_ticker = prev_ticker.lerp(*next_ticker, lerp_value);

            lerp_ticker
        }
    }

    fn timestamp_to_float_index(&self, timestamp: u64) -> f64 {
        if timestamp < self.start_timestamp {
            return 0.0;
        } else if timestamp
            >= self.start_timestamp + self.ticker_size * (self.tickers.len() - 1) as u64
        {
            return self.tickers.len() as f64 - 1.0;
        } else {
            let index =
                (timestamp as f64 - (self.start_timestamp as f64)) / self.ticker_size as f64;

            index
        }
    }

    fn timestamp_to_index(&self, timestamp: u64) -> usize {
        // check if timestamp less than start_timestamp
        // if so, return 0
        // else check if timestamp is greater than last timestamp using the length of the tickers
        // else calculate index from timestamp

        if timestamp < self.start_timestamp {
            0 as usize
        } else if timestamp
            >= self.start_timestamp + self.ticker_size * (self.tickers.len() - 1) as u64
        {
            self.tickers.len() - 1
        } else {
            let index = (timestamp - self.start_timestamp) / self.ticker_size;
            index as usize
        }
    }

    fn get_ticker_count(&self) -> usize {
        self.tickers.len()
    }

    fn add_ticker(&mut self, ticker: Ticker) {
        self.tickers.push(ticker);
    }
}

//tests
#[cfg(test)]
mod tests {
    use super::*;
    const START_TIMESTAMP: u64 = 1546300800;
    #[test]
    fn test_ticker_store_new() {
        let ticker_store = TickerStore::new(15, START_TIMESTAMP);
        assert_eq!(ticker_store.get_ticker_count(), 0);
    }

    #[test]
    fn test_ticker_store_add_ticker() {
        let mut ticker_store = TickerStore::new(15, START_TIMESTAMP);
        let ticker = Ticker {
            high: 1.0,
            low: 0.0,
        };
        ticker_store.add_ticker(ticker);
        assert_eq!(ticker_store.get_ticker_count(), 1);
    }

    #[test]
    fn test_ticker_store_get_ticker() {
        let mut ticker_store = TickerStore::new(15, START_TIMESTAMP);
        ticker_store.add_ticker(Ticker {
            high: 1.0,
            low: 0.0,
        });
        ticker_store.add_ticker(Ticker {
            high: 2.0,
            low: 0.0,
        });
        let ticker = ticker_store.get_ticker(START_TIMESTAMP);
        assert_eq!(ticker.high, 1.0);
        assert_eq!(ticker.low, 0.0);
        let ticker = ticker_store.get_ticker(START_TIMESTAMP + 5);
        assert_eq!(ticker.high, 1.0);
        assert_eq!(ticker.low, 0.0);
        let ticker = ticker_store.get_ticker(START_TIMESTAMP + 15);
        assert_eq!(ticker.high, 2.0);
        assert_eq!(ticker.low, 0.0);
        let ticker = ticker_store.get_ticker(START_TIMESTAMP + 16);
        assert_eq!(ticker.high, 2.0);
        assert_eq!(ticker.low, 0.0);
    }

    #[test]
    fn timestamp_to_float_index() {
        let mut ticker_store = TickerStore::new(15, START_TIMESTAMP);
        ticker_store.add_ticker(Ticker {
            high: 1.0,
            low: 0.0,
        });
        ticker_store.add_ticker(Ticker {
            high: 2.0,
            low: 0.0,
        });
        let index = ticker_store.timestamp_to_float_index(START_TIMESTAMP);
        assert_eq!(index, 0.0);
        let index = ticker_store.timestamp_to_float_index(START_TIMESTAMP + 5);
        assert_eq!(index > 0.0, true);
        assert_eq!(index < 0.5, true);
        let index = ticker_store.timestamp_to_float_index(START_TIMESTAMP + 15);
        assert_eq!(index, 1.0);

        let index = ticker_store.timestamp_to_float_index(START_TIMESTAMP + 16);
        assert_eq!(index, 1.0);
    }

    #[test]
    fn test_ticker_store_get_ticker_lerp() {
        let mut ticker_store = TickerStore::new(15, START_TIMESTAMP);
        ticker_store.add_ticker(Ticker {
            high: 1.0,
            low: 0.0,
        });
        ticker_store.add_ticker(Ticker {
            high: 2.0,
            low: 0.0,
        });
        let ticker = ticker_store.get_ticker_lerp(START_TIMESTAMP);
        assert_eq!(ticker.high, 1.0);
        assert_eq!(ticker.low, 0.0);
        let ticker = ticker_store.get_ticker_lerp(START_TIMESTAMP + 14);
        assert_eq!(ticker.high > 1.7, true);
        assert_eq!(ticker.high < 2.0, true);
        assert_eq!(ticker.low, 0.0);
    }

    fn timestamp_index_test_generic(ticker_size: u64, start_timestamp: u64) {
        let mut ticker_store = TickerStore::new(ticker_size, start_timestamp);
        let ticker = Ticker {
            high: 1.0,
            low: 0.0,
        };
        ticker_store.add_ticker(ticker);
        ticker_store.add_ticker(ticker);
        ticker_store.add_ticker(ticker);
        //if timestamp is within range (typical operation)
        let index = ticker_store.timestamp_to_index(start_timestamp);
        assert_eq!(index, 0);
        let index = ticker_store.timestamp_to_index(start_timestamp + (ticker_size * 0.1 as u64));
        assert_eq!(index, 0);
        let index = ticker_store.timestamp_to_index(start_timestamp + ticker_size);
        assert_eq!(index, 1);
        let index = ticker_store.timestamp_to_index(start_timestamp + (ticker_size * 2));
        assert_eq!(index, 2);

        //if timestamp is outside range
        let index = ticker_store.timestamp_to_index(start_timestamp - 1);
        assert_eq!(index, 0);
        let index = ticker_store.timestamp_to_index(start_timestamp + (ticker_size * 3));
        assert_eq!(index, 2);

        //if timestamp isn't a multiple of ticker_size, it should be to the nearest index
        let index = ticker_store.timestamp_to_index(start_timestamp - (ticker_size / 2));
        assert_eq!(index, 0);
        let index = ticker_store.timestamp_to_index(start_timestamp + (ticker_size) + 2);
        assert_eq!(index, 1);
        let index = ticker_store.timestamp_to_index(start_timestamp + (ticker_size * 2) + 1); //just after the last ticker
        assert_eq!(index, 2);
        let index = ticker_store
            .timestamp_to_index(start_timestamp + ((ticker_size * 2) - (ticker_size / 2))); //inbetween the last two tickers
        assert_eq!(index, 1);
    }

    #[test]
    fn test_ticker_store_timestamp_to_index() {
        timestamp_index_test_generic(15, START_TIMESTAMP);
        timestamp_index_test_generic(5, START_TIMESTAMP);
    }
}
