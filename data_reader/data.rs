
pub mod data {
    #![allow(dead_code)]
    use csv::Reader;
    use once_cell::sync::Lazy;
    use serde::Deserialize;
    
    pub const COLS: Lazy<Vec<String>> = Lazy::new(|| {
        ["Date", "Open", "High", "Low", "Close", "Adj Close", "Volume"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    });

    #[derive(Debug, Deserialize, Clone)]
    pub struct DataFeed {
        pub Date: Vec<String>,
        pub Open: Vec<f64>,
        pub High: Vec<f64>,
        pub Low: Vec<f64>,
        pub Close: Vec<f64>,
        #[serde(rename = "Adj Close")]
        pub Adj_Close: Vec<f64>,
        pub Volume: Vec<u64>,
    }
    impl DataFeed {

        pub fn new() -> Self {
            let ohlcv = DataFeed {
                Date: Vec::new(), 
                Open: Vec::new(), 
                High: Vec::new(), 
                Low: Vec::new(), 
                Close: Vec::new(), 
                Adj_Close: Vec::new(), 
                Volume: Vec::new(),
            };
            ohlcv
        }

        pub fn read_csv(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
            let mut reader = Reader::from_path(filename)?;
            
            let header = reader.headers()?;
            println!("{:?}", header);

            for line in reader.records() {
                let line = line?;
                self::DataFeed::parse_line(self, line)?;
            }
            
            Ok(())
        }
        
        fn parse_line(&mut self, line: csv::StringRecord) -> Result<(), Box<dyn std::error::Error>> {
            let date = line.get(0).ok_or("Missing Date")?.to_string();
            let open: f64 = line.get(1).ok_or("Missing Open")?.parse()?;
            let high: f64 = line.get(2).ok_or("Missing High")?.parse()?;
            let low: f64 = line.get(3).ok_or("Missing Low")?.parse()?;
            let close: f64 = line.get(4).ok_or("Missing Close")?.parse()?;
            let adj_close: f64 = line.get(5).ok_or("Missing Adj Close")?.parse()?;
            let volume: u64 = line.get(6).ok_or("Missing Volume")?.parse()?;
            
            self.Date.push(date);
            self.Open.push(open);
            self.High.push(high);
            self.Low.push(low);
            self.Close.push(close);
            self.Adj_Close.push(adj_close);
            self.Volume.push(volume); 
            Ok(())
        }

        pub fn print_ohlcv(&self, start: String, end: String) -> Result<(), Box<dyn std::error::Error>> {
            let start_idx = self.Date.iter().position(|d| *d == start).ok_or("Start date not found")?;
            let end_idx = self.Date.iter().position(|d| *d == end).ok_or("End date not found")?;
            if start_idx > end_idx {
                return Err("Start date must be before end date".into());
            }
            for idx in start_idx..=end_idx {
                println!(
                    "{} | {:<12} | {:<12} | {:<12} | {:<12} | {:<12} | {:<12} |",
                    self.Date[idx],
                    self.Open[idx],
                    self.High[idx],
                    self.Low[idx],
                    self.Adj_Close[idx],
                    self.Close[idx],
                    self.Volume[idx],
                );
            }
            Ok(())
        }

        pub fn get_ohlcv(&self) -> DataFeed {
            self.clone()
        }

        pub fn clear_ohclv(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            *self = DataFeed::new();
            Ok(())
        }

        pub fn get_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
            Ok(self.Date.len().try_into().unwrap())
        }
    }
}