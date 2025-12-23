

struct Account{
    acc_id : u8,
    balance : u64
}

struct AccountList{
    accounts : Vec<Account>,
    next_id : u8
}
enum Error {
    InsufficientBalance,
    InvalidAccount
}

impl AccountList{
    fn new() -> Self{
        Self{
           accounts : Vec::new(),
           next_id : 1
        }
    }
    
    fn add_account(&mut self , balance : u64) {
        let acc = Account{
            acc_id : self.next_id,
            balance : balance
        };
        self.next_id +=1;
        self.accounts.push(acc);
    }
    
    fn transfer(&mut self, amount : u64 , to : usize ,from : usize) -> Result<() ,Error>{
        if to >= self.accounts.len() || from >= self.accounts.len(){
            return Err(Error::InvalidAccount);
        }
        
        if self.accounts[from].balance < amount {
            return Err(Error::InsufficientBalance);
        }
        let (a, b) = if from < to {
            let (left, right) = self.accounts.split_at_mut(to);
            (&mut left[from], &mut right[0])
        } else {
            let (left, right) = self.accounts.split_at_mut(from);
            (&mut right[0], &mut left[to])
        };

        a.balance -= amount;
        b.balance += amount;

        Ok(())
    }

    fn total_balance(&self) -> u64{
        self.accounts.iter().map(|x| x.balance).sum()
    }   
}