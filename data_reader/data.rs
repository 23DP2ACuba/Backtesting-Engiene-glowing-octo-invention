// pub fn process_data(){
//     println!("processed sata");
// }
#[allow(dead_code)]
pub mod data {
    use csv::Reader;
    use once_cell::sync::Lazy;

    pub const COLS: Lazy<Vec<String>> = Lazy::new(|| {
        ["Date", "Open", "High", "Low", "Close", "Adj Close", "Volume"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    });

    pub fn csv_reader(filename: &'static str) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = Reader::from_path(filename)?;
    
        let header = reader.headers()?;
        println!("{:?}", header);
        
        Ok(())
    }

    fn parse_line(line: &String) -> Result<(), Box<dyn std::error::Error>>{
        Ok(())
    }
}