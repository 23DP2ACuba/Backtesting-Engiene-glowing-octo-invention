mod data_reader;
mod engiene;
mod indicators;
mod strategy;
use std::collections::HashMap;
use data_reader::data::data::DataFeed;
use engiene::engiene::backtest::{PramVal, Prams, Backtest};
use indicators::indicators::Indicators;
use crate::strategy::strategy::Strategy;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_feed = DataFeed::new();
    let csv_file = "BTC-USD.csv"; // Replace with actual CSV file path
    data_feed.read_csv(csv_file)?;
    
    // Print sample OHLCV data for verification
    if data_feed.Date.is_empty() {
        return Err("No data available in DataFeed".into());
    }

    // Create strategy with the data feed
    let mut strategy = Strategy::new(data_feed);
    
    // Set custom parameters
    let mut params = Prams {
        data: HashMap::new(),
    };
    params.data.insert("balance".to_string(), PramVal::Double(100000.0));
    params.data.insert("commission".to_string(), PramVal::Double(0.002));
    params.data.insert("slippage".to_string(), PramVal::Double(0.0005));
    params.data.insert("sizer".to_string(), PramVal::Double(0.02));
    
    strategy.set_params(params)?;
    
    // Run the strategy with SMA period of 10
    strategy.next(12, 13)?;
    
    // Run the backtest
    strategy.run()?;
    
    // Display statistics
    strategy.stats()?;
    
    Ok(())
}