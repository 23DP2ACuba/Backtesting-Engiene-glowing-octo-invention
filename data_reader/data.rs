// pub fn process_data(){
//     println!("processed sata");
// }
#[allow(dead_code)]
pub mod data {
    use csv::Reader;
    use once_cell::sync::Lazy;
    use serde::Deserialize;

    pub const COLS: Lazy<Vec<String>> = Lazy::new(|| {
        ["Date", "Open", "High", "Low", "Close", "Adj Close", "Volume"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    });

    #[derive(Debug, Deserialize)]
    pub struct OHLCV {
        pub Date: Vec<String>,
        pub Open: Vec<f64>,
        pub High: Vec<f64>,
        pub Low: Vec<f64>,
        pub Close: Vec<f64>,
        #[serde(rename = "Adj Close")]
        pub Adj_Close: Vec<f64>,
        pub Volume: Vec<u64>,
        pub Idx: i64,
    }

    pub struct DataFeed {
        ohlcv: OHLCV,
    }
    impl DataFeed {
        pub fn new() -> Self{
            DataFeed {
                ohlcv: OHLCV { 
                    Date: Vec::new(), 
                    Open: Vec::new(), 
                    High: Vec::new(), 
                    Low: Vec::new(), 
                    Close: Vec::new(), 
                    Adj_Close: Vec::new(), 
                    Volume: Vec::new(),
                    Idx: 0,
                }
            }
        }
        pub fn csv_reader(&mut self, filename: &'static str) -> Result<(), Box<dyn std::error::Error>> {
            let mut reader = Reader::from_path(filename)?;
            
            let header = reader.headers()?;
            println!("{:?}", header);

            for line in reader.records() {
                let line = line?;
                self::DataFeed::parse_line(self, line)?;
                
            }
            
            Ok(())
        }
        

        fn parse_line(&mut self, line: csv::StringRecord) -> Result<(), Box<dyn std::error::Error>>{
            let date = line.get(0).unwrap_or_default().to_string();
            let open: f64 = line.get(0).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let high: f64 = line.get(0).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let low: f64 = line.get(0).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let close: f64 = line.get(0).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let adj_close: f64 = line.get(0).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let volume: u64 = line.get(0).unwrap_or_default().to_string().parse().unwrap_or(0);
            

            self.ohlcv.Date.push(date);
            self.ohlcv.Open.push(open);
            self.ohlcv.High.push(high);
            self.ohlcv.Low.push(low);
            self.ohlcv.Close.push(close);
            self.ohlcv.Adj_Close.push(adj_close);
            self.ohlcv.Volume.push(volume); 
            self.ohlcv.Idx += 1;
            Ok(())
        }

        pub fn printOHLCV(&self, start: String, end: String) -> Result<(), Box<dyn std::error::Error>> {
            let last_date = match self.ohlcv.Date.last(){
                Some(last_val) => last_val,
                None => &String::new(),
            };
            if start < end && end <= *last_date{
                for line in self.ohlcv.idx
            }
            Ok(())
        }

        pub fn clearOHCLV() -> Result<(), Box<dyn std::error::Error>> {
            DataFeed::new();
            Ok(())
        }

    }

    
}