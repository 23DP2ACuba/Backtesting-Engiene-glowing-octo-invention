// Fixed closing logic to use actual position quantities
// Retained previous fixes: dynamic quantities, error handling, run/stats methods
pub mod strategy {
    use crate::Indicators;
    use crate::{Backtest, Prams};
    use crate::DataFeed;


    pub struct Strategy {
        pub backtest: Backtest,
    }

    impl Strategy {
        pub fn new(data: DataFeed) -> Self {
            Strategy {
                backtest: Backtest::new(data),
            }
        }
        pub fn set_params(&mut self, prams: Prams)  -> Result<(), Box<dyn std::error::Error>>{
            self.backtest.set_params(prams)
        }
        pub fn next(&mut self, period1: usize, period2: usize) -> Result<(), Box<dyn std::error::Error>> {
            let mut indicators = Indicators {
                ohlcv: self.backtest.ohlcv.get_ohlcv(),
            };

            let ssma = indicators.SMA(period1 as i64)?;
            let lsma = indicators.SMA(period2 as i64)?;

            let ohlcv = self.backtest.ohlcv.get_ohlcv();
            let open = &ohlcv.Open;
            let date = &ohlcv.Date;

            for i in period2..lsma.len() {
                let long_signal = ssma[i - 1] > lsma[i - 1] && ssma[i - 2] < lsma[i - 2];
                let short_signal = ssma[i - 1] < lsma[i - 1] && ssma[i - 2] > lsma[i - 2];

                let dt = &date[i];
                let price = open[i];
                let quantity = (self.backtest.sizer * self.backtest.balance / price).max(0.0);

                if long_signal {
                    self.backtest.buy(dt.clone(), price, quantity)?;
                } else if short_signal {
                    self.backtest.sell(dt.clone(), price, quantity)?;
                } else if self.backtest.position {
                    let close_quantity = if self.backtest.is_long {
                        self.backtest.long_quantity
                    } else {
                        self.backtest.short_quantity
                    };
                    self.backtest.close(&dt.clone(), &price, close_quantity)?;
                }
            }

            Ok(())
        }

        pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.backtest.run()
        }

        pub fn stats(&self) -> Result<(), Box<dyn std::error::Error>> {
            self.backtest.stats()
        }
    }
}