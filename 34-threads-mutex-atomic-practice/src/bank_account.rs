use std::sync::{Arc, Mutex};
use std::thread;

pub struct BankAccount {
    balance: u32,
}

impl BankAccount {
    pub fn new(initial_balance: u32) -> Self {
        Self {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: u32) {
        if self.balance >= amount {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> u32 {
        self.balance
    }
}

/// Пять потоков пополняют счёт, а пять потоков снимают деньги.
pub fn run() -> u32 {
    let account = Arc::new(Mutex::new(BankAccount::new(1_000)));
    let mut handles = vec![];

    for _ in 0..5 {
        let account_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                account_clone.lock().unwrap().deposit(50);
            }
        });
        handles.push(handle);
    }

    for _ in 0..5 {
        let account_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                account_clone.lock().unwrap().withdraw(30);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    account.lock().unwrap().balance()
}

#[cfg(test)]
mod tests {
    use super::{BankAccount, run};

    #[test]
    fn calculates_final_balance() {
        assert_eq!(run(), 2_000);
    }

    #[test]
    fn does_not_withdraw_more_than_balance() {
        let mut account = BankAccount::new(20);
        account.withdraw(30);
        assert_eq!(account.balance(), 20);
    }
}
