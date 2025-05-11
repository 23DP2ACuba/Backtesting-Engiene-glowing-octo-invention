pub mod backtest {
    #![allow(dead_code)]
    use crate::data_reader::data::data::DataFeed;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum PramVal {
        Double(f64),
        Int(i32),
        Bool(bool),
    }

    pub struct Prams {
        pub data: HashMap<String, PramVal>,
    }
    pub struct Backtest {
        pub ohlcv: DataFeed,
        pub balance: f64,
        pub init_balance: f64,
        pub final_balance: f64,
        pub commission: f64,
        pub total_commission: f64,
        pub slippage: f64,
        pub total_slippage: f64,
        pub risk: f64,
        pub position: bool,
        pub is_short: bool,
        pub is_long: bool,
        pub quantity: f64,
        pub long_quantity: f64,
        pub short_quantity: f64,
        pub sizer: f64,
        pub long_entry_price: f64,
        pub short_entry_price: f64,

    }


    impl Backtest {
        pub fn new(data: DataFeed) -> Self {
            Backtest {
                ohlcv: data,
                balance: 100000.0,
                init_balance: 0.0,
                final_balance: 0.0,
                commission: 0.0,
                total_commission: 0.0,
                risk: 0.0,
                position: false,
                is_long: false,
                is_short: false,
                quantity: 0.0,
                long_quantity: 0.0,
                short_quantity: 0.0,
                sizer: 0.0,
                slippage: 0.0,
                total_slippage: 0.0,
                long_entry_price: 0.0,
                short_entry_price: 0.0,
            }
        }

        pub fn set_balance(&mut self, cash: f64) {
            self.balance = cash;
            self.init_balance = cash;
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

        pub fn set_params(&mut self, prams: Prams) -> Result<(), Box<dyn std::error::Error>> {
            for (key, val) in &prams.data {
                match (key.as_str(), val) {
                    ("balance", PramVal::Double(v)) => self.set_balance(*v),
                    ("commission", PramVal::Double(v)) => self.set_comission(*v),
                    ("slippage", PramVal::Double(v)) => self.set_slippage(*v),
                    ("sizer", PramVal::Double(v)) => self.set_sizer(*v),
                    _ => {}
                }
            }
            Ok(())
        }

        fn calculate_comm(&mut self, price: f64, quantity: f64) -> Result<f64, Box<dyn std::error::Error>> {
            if price <= 0.0 || quantity <= 0.0 {
                return Err("Invalid price or quantity".into());
            }
            let comm = self.commission * price * quantity;
            self.total_commission += comm;
            Ok(comm)
        }

        fn calculate_slippage(&mut self, price: f64, quantity: f64) -> f64 {
            if price <= 0.0 || quantity <= 0.0 {
                return 0.0;
            }
            let slip = price * self.slippage * quantity;
            self.total_slippage += slip;
            slip
        }

        pub fn log(&mut self, record: String) {
            println!("{:?}", record);
        }

        pub fn buy(&mut self, date: String, price: f64, quantity: f64) -> Result<(), Box<dyn std::error::Error>> {
            if quantity <= 0.0 || price <= 0.0 {
                return Err("Invalid quantity or price".into());
            }
            let adjusted_price = price * (1.0 + self.slippage);
            let comm = self.calculate_comm(adjusted_price, quantity)?;
            let slip = self.calculate_slippage(adjusted_price, quantity);
            
            if !self.position {
                self.is_long = true;
                self.position = true;
                self.long_quantity = quantity;
                self.long_entry_price = adjusted_price;
                self.balance -= adjusted_price * quantity + comm + slip;
            } else if self.is_long {
                let total_cost = self.long_entry_price * self.long_quantity + adjusted_price * quantity;
                self.long_quantity += quantity;
                self.long_entry_price = total_cost / self.long_quantity;
                self.balance -= adjusted_price * quantity + comm + slip;
            } else if self.is_short {
                let short_pnl = (self.short_entry_price - adjusted_price) * self.short_quantity;
                self.balance += short_pnl - comm - slip;
            }

            self.log(format!("BUY {}, {}, {}", date, adjusted_price, quantity));
            Ok(())
        }

        pub fn sell(&mut self, date: String, price: f64, quantity: f64) -> Result<(), Box<dyn std::error::Error>> {
            if quantity <= 0.0 || price <= 0.0 {
                return Err("Invalid quantity or price".into());
            }
            let comm = self.calculate_comm(price, quantity)?;
            let _slip = self.calculate_slippage(price, quantity);
            let adjusted_price = price * (1.0 - self.slippage);

            if !self.position {
                self.is_short = true;
                self.position = true;
                self.short_quantity = quantity;
                self.balance += adjusted_price * quantity - comm;
            } else if self.is_short {
                self.short_quantity += quantity;
                self.balance += adjusted_price * quantity - comm;
            } else if self.is_long && quantity > self.long_quantity {
                self.is_long = false;
                self.is_short = true;
                self.short_quantity = quantity - self.long_quantity;
                self.long_quantity = 0.0;
                self.balance += adjusted_price * quantity - comm;
            } else if self.is_long && quantity <= self.long_quantity {
                self.long_quantity -= quantity;
                self.balance += adjusted_price * quantity - comm;
                if self.long_quantity <= 0.0 {
                    self.is_long = false;
                    self.position = false;
                }
            }
            self.log(format!("SELL {}, {}, {}", date, adjusted_price, quantity));
            Ok(())
        }

        pub fn close(&mut self, date: &String, price: &f64, quantity: f64) -> Result<(), Box<dyn std::error::Error>> {
            if quantity <= 0.0 || *price <= 0.0 {
                return Err("Invalid quantity or price".into());
            }
            let comm = self.calculate_comm(*price, quantity)?;
            let _slip = self.calculate_slippage(*price, quantity);
            let adjusted_price = if self.is_long {
                price * (1.0 - self.slippage)
            } else {
                price * (1.0 + self.slippage)
            };

            if self.is_long {
                let close_quantity = quantity.min(self.long_quantity);
                self.long_quantity -= close_quantity;
                self.balance += adjusted_price * close_quantity - comm;
                if self.long_quantity <= 0.0 {
                    self.is_long = false;
                    self.position = false;
                    self.long_quantity = 0.0;
                }
            } else if self.is_short {
                let close_quantity = quantity.min(self.short_quantity);
                self.short_quantity -= close_quantity;
                self.balance -= adjusted_price * close_quantity - comm;
                if self.short_quantity <= 0.0 {
                    self.is_short = false;
                    self.position = false;
                    self.short_quantity = 0.0;
                }
            }
            self.log(format!("CLOSE {}, {}, {}", date, adjusted_price, quantity));
            Ok(())
        }

        pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            println!("running backtest...");
            println!("starting balance: {}", self.balance);


            if self.is_long && self.long_quantity > 0.0 {
                self.close(self.ohlcv.get_ohlcv().Date.last().ok_or("No data available")?,
                          self.ohlcv.get_ohlcv().Close.last().ok_or("No data available")?,
                          self.long_quantity)?;
            } else if self.is_short && self.short_quantity > 0.0 {
                self.close(self.ohlcv.get_ohlcv().Date.last().ok_or("No data available")?,
                          self.ohlcv.get_ohlcv().Close.last().ok_or("No data available")?,
                          self.short_quantity)?;
            }
            self.final_balance = self.balance;
            println!("final balance: {}", self.balance);
            Ok(())
        }

        pub fn stats(&self) -> Result<(), Box<dyn std::error::Error>> {
            println!("...Stats...");
            let profitable = self.final_balance > self.init_balance;
            let pl = if profitable { "Profit" } else { "Loss" };
            println!("{}: {}", pl, self.final_balance - self.init_balance);
            println!("Return: {}", (self.final_balance - self.init_balance) / self.init_balance * 100.0);
            println!("Commission: {}", self.total_commission);
            println!("Slippage: {}", self.total_slippage);
            Ok(())
        }
    }
}