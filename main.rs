mod data_reader;
mod engn_code;
mod indicators;
use csv::Reader;
use data_reader::data::data::DataFeed;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut df = DataFeed::new();
    df.csv_reader("BTC-USD.csv")?;
    Ok(())
}


