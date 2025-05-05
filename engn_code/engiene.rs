

pub mod backtest {
    #![allow(dead_code)]
    use crate::data_reader::data::data::DataFeed;

    pub struct Backtest {
        pub ohlcv: DataFeed,
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

    pub trait IteratorLike {
        fn next(&mut self);
    }
    impl IteratorLike for Backtest {
        fn next(&mut self) {
            println!("Next step executed.");
        }
    }

    impl Backtest {
        pub fn new(data: DataFeed) -> Self{
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
            let comm = self.commission * price * quantity as f64;
            self.total_commission += comm;
            Ok(comm)
        }
        pub fn log(&mut self, record: String) {
            println!("{:?}", record);
        }

        pub fn buy(&mut self, date: String, price: f64, quantity: u16) -> Result<(), Box<dyn std::error::Error>>{ 
            let comm =  self.calculate_comm(price, quantity)?;

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

            self.log(format!("BUY {}, {}, {}",  date, price, quantity));
            Ok(())
        }

        pub fn sell(&mut self, date: String, price: f64, quantity: u16) -> Result<(), Box<dyn std::error::Error>> {
            let comm =  self.calculate_comm(price, quantity)?;
            
            if !self.position {
                self.is_short = true;
                self.position = true;
                self.short_quantity = quantity;
                self.balance += price * quantity as f64 - comm;
            } else if self.is_short{
                self.short_quantity += quantity;
                self.balance += price * quantity as f64 - comm;
            } else if self.is_long && quantity > self.long_quantity {
                self.is_long = false;
                self.is_short = true;
                self.short_quantity = quantity - self.long_quantity;
                self.long_quantity = 0;
                self.balance += price * quantity as f64 - comm;
            } else if self.is_long && quantity <= self.long_quantity {
                self.long_quantity -= quantity;
                self.balance -= price * quantity as f64 + comm;
            }
            self.log(format!("SELL {}, {}, {}",  date, price, quantity));
            Ok(())
        }

        pub fn close(&mut self, date: &String, price: &f64, quantity: u16) -> Result<(), Box<dyn std::error::Error>> {
            let comm =  self.calculate_comm(*price, quantity)?;

            if self.is_long {
                self.is_long = false;
                self.position = false;
                self.long_quantity -= quantity;
                self.balance += price * quantity as f64 - comm;
            }else if self.is_short {
                self.is_short = false;
                self.position = false;
                self.short_quantity -= quantity;
                self.balance += price * quantity as f64 + comm;
            }
            self.log(format!("CLOSE {}, {}, {}",  date, price, quantity));
            Ok(())
        }

        pub fn run(&mut self) {
            println!("running backtest...");
            println!("starting balance: {}", self.balance);

            self.init_balance = self.balance;
            self.next();

            if self.is_long {
                self.close(self.ohlcv.get_ohlcv().Date.last().unwrap(), self.ohlcv.get_ohlcv().Close.last().unwrap(), self.long_quantity);
            } else if self.is_long {
                self.close(self.ohlcv.get_ohlcv().Date.last().unwrap(), self.ohlcv.get_ohlcv().Close.last().unwrap(), self.short_quantity);
            }
            self.final_balance = self.balance;
            println!("final balance: {}", self.balance);
        
        }

        pub fn stats(&mut self) {
            println!("...Stats...");
            let profitable = self.final_balance > self.init_balance;
            let mut pl = "Profit".to_string();
            if !profitable {pl = "Loss".to_string()}
            println!("{pl}: {}", (self.final_balance - self.init_balance));
            println!("Return: {}", (self.final_balance - self.init_balance) / self.init_balance);
            println!("Commission: {}", self.total_commission);

        }

        

    }
}