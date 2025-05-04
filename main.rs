mod data_reader;
mod engn_code;
mod indicators;
use csv::Reader;
use data_reader::data::data::DataFeed;
use engn_code::engiene::backtest::Backtest;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut df = DataFeed::new();
    
    df.read_csv("BTC-USD.csv")?;

    let size = df.get_size()?;
    println!("{}", size);

    df.print_ohlcv("2024-06-26".to_string(), "2024-06-28".to_string())?;
    
    let ohlcv = df.get_ohlcv();
    println!("{:?}", ohlcv.Date[0]);

    Backtest::new(ohlcv);
    Ok(())
}


