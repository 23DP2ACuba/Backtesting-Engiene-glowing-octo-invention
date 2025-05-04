
use crate::data_reader;

pub mod backtest {
    #![allow(dead_code)]
    use crate::data_reader::data::data::OHLCV;

    pub struct Backtest {
        pub ohlcv: OHLCV,
        balance: f64,
        init_balance: f64,
        final_balance: f64,
        commission: f64,
        total_commission: f64,
        slippage: f64,
        total_slippage: f64,
        risk: f64,
        position: bool,
        is_short: bool,
        is_long: bool,
        quantity: u16,
        long_quantity: u16,
        short_quantity: u16,
        sizer: f64,

    }

    impl Backtest {
        pub fn new(data: OHLCV) -> Self{
            Backtest {
                ohlcv: data,
                balance: 0.0,
                init_balance: 0.0,
                final_balance: 0.0,
                commission: 0.0,
                total_commission: 0.0,
                risk: 0.0,
                position: false,
                is_long: false,
                is_short: false,
                quantity: 0,
                long_quantity: 0,
                short_quantity: 0,
                sizer: 0.0,
                slippage: 0.0,
                total_slippage: 0.0,
            }
        }

        pub fn set_balance(&mut self, cash: f64) {
            self.balance = cash;
        }

        pub fn set_comission(&mut self, commission: f64) {
            self.commission = commission;
        }

        pub fn set_slippage(&mut self, slippage: f64) {
            self.slippage = slippage;
        }

        pub fn set_sizer(&mut self, sizer: f64) {
            self.sizer = sizer;
        }

        fn calculate_comm(&mut self, price: f64, quantity: u16) -> Result<f64, Box<dyn std::error::Error>> {
            let mut comm = self.commission * price * quantity as f64;
            self.total_commission += comm;
            Ok(comm)
        }

        pub fn buy(&mut self, date: String, price: f64, quantity: u16) -> Result<(), Box<dyn std::error::Error>>{
            
            let mut comm =  self.calculate_comm(price, quantity)?;
            

            if !self.position {
                self.is_long = true;
                self.position = true;
                self.long_quantity = quantity;
                self.balance -= price * quantity as f64 + comm;
            } else if self.is_long {
                self.long_quantity += quantity;
                self.balance -= price * quantity as f64 + comm;
            } else if self.is_short && quantity > self.short_quantity {
                self.is_short = false;
                self.is_long = true;
                self.long_quantity = quantity -self.short_quantity;
                self.short_quantity = 0;
                self.balance -= price * quantity as f64 + comm; 
            } else if self.is_short && quantity <= self.short_quantity {
                self.short_quantity -= quantity;
                self.balance += price * quantity as f64 - comm;
            }

            self.log(format!("close {date} {price} {quantity}"), );
            Ok(())
        }

        pub fn log(record: &str) {
            println!("{:?}", record);
        }

        pub fn brain_damage() -> ! { loop{ println!("H1tler:)") } }


    }
}