use crate::data_reader::data::data::DataFeed;
pub struct Indicators {
    pub ohlcv: DataFeed,
} 
impl Indicators {
    
    pub fn SMA(&mut self, period: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let data = &self.ohlcv.get_ohlcv().Close;
        if period as usize > data.len() {
            return Err("Period exceeds data length".into());
        }
        let mut sma = vec![0.0; self.ohlcv.get_ohlcv().Date.len()];
        let mut sum = 0.0;
        for i in 0..period {
            sum += data[i as usize];
        }
        sma[(period - 1) as usize] = sum / period as f64;

        for i in period..data.len() as i64 {
            sum -= data[(i-period) as usize];
            sum += data[i as usize];
            sma[i as usize] = sum / period as f64;
        }

        Ok(sma)
    }

    pub fn EMA(&mut self, period: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let data = &self.ohlcv.get_ohlcv().Close;
        if period as usize > data.len() {
            return Err("Period exceeds data length".into());
        }
        let mut ema = vec![0.0; self.ohlcv.get_ohlcv().Date.len()];

        let mut sum = 0.0;
        for i in 0..period {
            sum += data[i as usize];
        }

        ema[(period - 1) as usize] = sum / period as f64;

        for i in period..data.len() as i64 {
            let i = i as usize;
            ema[i] = (data[i] * (2.0 / (period + 1) as f64)) + (ema[i - 1] * (1.0 - (2.0 / (period + 1) as f64)));
        }
        Ok(ema)
    }

    fn _MACD(&mut self, fast: i64, slow: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let ohlcv = self.ohlcv.get_ohlcv();
        let close = &ohlcv.Close;
        let len = close.len();
    
        let ema_fast = self.EMA(fast)?;
        let ema_slow = self.EMA(slow)?;
    
        if ema_fast.len() != len || ema_slow.len() != len {
            return Err("EMA lengths do not match OHLCV length".into());
        }
    
        let macd: Vec<f64> = ema_fast.iter()
            .zip(ema_slow.iter())
            .map(|(short, long)| short - long)
            .collect();
    
        Ok(macd)
    }

    fn _SIGNAL(&mut self, period: i64, fast: i64, slow: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let macd_line = self._MACD(fast, slow)?; 
        let signal_line = Self::ema_from_vec(&macd_line, period)?;
        Ok(signal_line)
    }

    fn ema_from_vec(data: &Vec<f64>, period: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        if data.len() < period as usize {
            return Err("Not enough data to compute EMA".into());
        }
        let mut ema = vec![0.0; data.len()];
        let k = 2.0 / (period as f64 + 1.0);
        
        // SMA for initial EMA value
        let sma: f64 = data[..period as usize].iter().sum::<f64>() / period as f64;
        ema[period as usize - 1] = sma;
        
        for i in period as usize..data.len() {
            ema[i] = (data[i] - ema[i - 1]) * k + ema[i - 1];
        }
        
        Ok(ema)
    }

    pub fn MACD(&mut self, period: i64, fast: i64, slow: i64) -> Result<(Vec<f64>, Vec<f64>), Box<dyn std::error::Error>> {
        let macd = self._MACD(fast, slow)?;
        let signal = self._SIGNAL(period, fast, slow)?;
        Ok((macd, signal))
    }

pub fn ATR(&mut self, period: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let ohlcv = self.ohlcv.get_ohlcv();
    let close = &ohlcv.Close;
    let high = &ohlcv.High;
    let low = &ohlcv.Low;
    let len = close.len();
    let mut atr = vec![0.0; len];
    
    let mut tr = vec![0.0; len];
    for i in 1..len {
        let prev_close = close[i - 1];
        tr[i] = f64::max(
            high[i] - low[i],
            f64::max(
                high[i] - prev_close,
                prev_close - low[i]
            )
        );
    }
    
    let mut sum = 0.0;
    for i in 1..=period as usize {
        sum += tr[i];
        if i == period as usize {
            atr[i] = sum / period as f64;
        }
    }
    
    for i in (period as usize + 1)..len {
        atr[i] = (atr[i - 1] * (period as f64 - 1.0) + tr[i]) / period as f64;
    }
    
    Ok(atr)
}

    pub fn BBANDS(&mut self, period: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let data = self.ohlcv.get_ohlcv().Close;
        let sma = self.SMA(period)?;
        let mut bbands = vec![0.0; self.ohlcv.get_ohlcv().Date.len()];
        let mut stddev = vec![0.0; self.ohlcv.get_ohlcv().Date.len()];

        for i in period..data.len() as i64 {
            let mut sum = 0.0;
            for j in (i - period)..i {
                let diff = data[j as usize] - sma[i as usize];
                sum += diff * diff;
            }
            stddev[i as usize] = (sum / period as f64).sqrt();
        }
    
        for i in 0..data.len() {
            if stddev[i] == 0.0 {
                println!("Warning: Standard deviation is zero at index {}, setting BBANDS to 0.0", i);
                bbands[i] = 0.0;
            } else {
                bbands[i] = (data[i] - sma[i]) / (2.0 * stddev[i]);
            }
        }
    
        Ok(bbands)
    }

    pub fn STOCHASTIC(&mut self, period: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let data = self.ohlcv.get_ohlcv().Close;
        let high = &self.ohlcv.get_ohlcv().High;
        let low = &self.ohlcv.get_ohlcv().Low;
        let mut stochastic = vec![0.0; self.ohlcv.get_ohlcv().Date.len()];
        let mut min = 0.0;
        let mut max = 0.0;
        
        for i in period..data.len() as i64 {
            min = low[(i-period) as usize];
            max = high[(i-period) as usize];

            for j in (i-period)..i {
                if low[j as usize] < min {
                    min = low[j as usize];
                }
                if high[j as usize] > max {
                    max = high[j as usize];
                }
            }
            stochastic[i as usize] = if max != min {
                (data[i as usize] - min) / (max - min) * 100.0
            } else {
                0.0
            };
        }
        Ok(stochastic)
    }

    pub fn WILLIAMS(&mut self, period: i64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let data = self.ohlcv.get_ohlcv().Close;
        let mut williams = vec![0.0; self.ohlcv.get_ohlcv().Date.len()];
        let mut min = 0f64;
        let mut max = 0f64;
        
        for i in period..data.len() as i64 {
            min = data[(i-period) as usize];
            max = data[(i-period) as usize];

            for j in (i-period)..i {
                if data[j as usize] < min {
                    min = data[j as usize];
                } else if data[j as usize] > max {
                    max = data[j as usize];
                }
            }
            williams[i as usize] = if max != min {
                (max - data[i as usize]) / (max - min)
            } else {
                0.0
            };
        }
        Ok(williams)
    }
}