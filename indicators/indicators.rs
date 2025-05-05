use crate::data_reader::data::data::DataFeed;
pub struct Indicators {
    pub ohlcv: DataFeed,
} 
impl Indicators {
    pub fn SMA(&mut self, period: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let mut sma = vec![0.0; self.ohlcv.get_ohlcv().Date.len()];
        let data = &self.ohlcv.get_ohlcv().Close;
        let mut sum = 0.0;
        for i in 0..period{
            sum += data[i as usize];
        }
        sma[(period - 1) as usize] = sum / period as f64;

        for i in period..data.len() as i64 {
            sum -= data[(i-period) as usize];
            sum += data[i as usize];
            sma[i as usize] = sum /period as f64;
        }

        Ok(sma)
    }
}