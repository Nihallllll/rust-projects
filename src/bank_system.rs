struct Account {
    owner : String,
    balance : u64
}
enum Messages{
    NotValidAmount(String),
    LowFunds(String)
}
impl Account {
    fn new(owner : &str , balance : u64) -> Self{
        Self{
            owner : owner.to_string(),
            balance
        }
    }

    fn get_balance(&self) -> u64 {
        self.balance
    }

    fn deposite(&mut self,amount :u64) -> Result<(),Messages>{
        if amount == 0 {
            return Err(Messages::NotValidAmount("Add valid amount".to_string()));
        }
        self.balance = self.balance.checked_add(amount).ok_or(Messages::NotValidAmount("Overflow".to_string()))?;
        Ok(())
    }

    fn withdraw(&mut self, amount : u64) -> Result<(),Messages> {
        if  amount > self.balance {
            return Err(Messages::LowFunds("Add valid amount".to_string()));
        }
        if  amount == 0 {
            return Err(Messages::NotValidAmount("Add valid amount".to_string()));
        }
        self.balance =self.balance.checked_sub(amount).unwrap();
        Ok(())
    }
}