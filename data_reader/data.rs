// pub fn process_data(){
//     println!("processed sata");
// }

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
    pub struct OHLCV {
        pub Date: Vec<String>,
        pub Open: Vec<f64>,
        pub High: Vec<f64>,
        pub Low: Vec<f64>,
        pub Close: Vec<f64>,
        #[serde(rename = "Adj Close")]
        pub Adj_Close: Vec<f64>,
        pub Volume: Vec<u64>,
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
                }
            }
        }

        pub fn read_csv(&mut self, filename: &'static str) -> Result<(), Box<dyn std::error::Error>> {
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
            let open: f64 = line.get(1).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let high: f64 = line.get(2).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let low: f64 = line.get(3).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let close: f64 = line.get(4).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let adj_close: f64 = line.get(5).unwrap_or_default().to_string().parse().unwrap_or(0.0);
            let volume: u64 = line.get(6).unwrap_or_default().to_string().parse().unwrap_or(0);
            

            self.ohlcv.Date.push(date);
            self.ohlcv.Open.push(open);
            self.ohlcv.High.push(high);
            self.ohlcv.Low.push(low);
            self.ohlcv.Close.push(close);
            self.ohlcv.Adj_Close.push(adj_close);
            self.ohlcv.Volume.push(volume); 
            Ok(())
        }

        pub fn print_ohlcv(&self, start: String, end: String) -> Result<(), Box<dyn std::error::Error>> {

            let last_date = match self.ohlcv.Date.last(){
                Some(last_val) => last_val,
                None => &String::new(),
            };

            if start < end && end <= *last_date{
                let mut idx: usize = self.ohlcv.Date.iter().position(|d| *d == start).unwrap() -1;

                let mut date = self.ohlcv.Date[idx].clone();
                while date != end {
                    idx += 1;
                    date = self.ohlcv.Date[idx].clone();

                    println!("{} | {:<12} | {:<12} | {:<12} | {:<12} | {:<12} | {:<12} |", 
                    date, 
                    self.ohlcv.Open[idx],
                    self.ohlcv.High[idx], 
                    self.ohlcv.Low[idx],
                    self.ohlcv.Adj_Close[idx],
                    self.ohlcv.Close[idx],
                    self.ohlcv.Volume[idx],
                );
                }
                
            }

            Ok(())
        }

        pub fn get_ohlcv(&self) -> OHLCV {
            self.ohlcv.clone()
        }

        pub fn clear_ohclv(&self) -> Result<(), Box<dyn std::error::Error>> {
            DataFeed::new();
            Ok(())
        }

        pub fn get_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
            Ok(self.ohlcv.Date.len().try_into().unwrap())
        }

    }

    
}