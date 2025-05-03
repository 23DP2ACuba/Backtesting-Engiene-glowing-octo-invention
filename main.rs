mod data_reader;
mod engn_code;
mod indicators;
use csv::Reader;
use data_reader::data::data::csv_reader;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    csv_reader("BTC-USD.csv")?;
    Ok(())
}


