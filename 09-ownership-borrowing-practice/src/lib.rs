/// Банковский аккаунт.
///
/// При уничтожении аккаунта `Drop` проверяет, что весь остаток был перенесён.
pub struct Account {
    pub name: String,
    pub balance: i64,
}

impl Account {
    pub fn new(name: &str, balance: i64) -> Self {
        Self {
            name: name.to_owned(),
            balance,
        }
    }
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.balance == 0 {
            println!("Аккаунт '{}' успешно уничтожен", self.name);
        } else {
            println!(
                "Предупреждение: аккаунт '{}' уничтожен с балансом {}",
                self.name, self.balance
            );
        }
    }
}

/// Банк, владеющий своими аккаунтами.
pub struct Bank {
    pub accounts: Vec<Account>,
    pub credit_rate: u32,
    pub debit_rate: u32,
}

impl Bank {
    pub fn new(accounts: Vec<Account>, credit_rate: u32, debit_rate: u32) -> Self {
        Self {
            accounts,
            credit_rate,
            debit_rate,
        }
    }
}

/// Печатает баланс, не забирая владение аккаунтом.
pub fn print_balance(account: &Account) {
    println!("Баланс аккаунта '{}': {}", account.name, account.balance);
}

/// Переводит `amount` между аккаунтами через эксклюзивные ссылки.
///
/// Отрицательный `amount` выполняет перевод в обратном направлении.
///
/// ```
/// use lesson_09_ownership_borrowing_practice::{Account, transfer_funds};
///
/// let mut from = Account::new("Отправитель", 500);
/// let mut to = Account::new("Получатель", 100);
/// transfer_funds(&mut from, &mut to, 200);
/// assert_eq!(from.balance, 300);
/// assert_eq!(to.balance, 300);
/// # from.balance = 0;
/// # to.balance = 0;
/// ```
pub fn transfer_funds(from: &mut Account, to: &mut Account, amount: i64) {
    from.balance -= amount;
    to.balance += amount;
}

/// Переносит остаток и уничтожает аккаунт, принятый по значению.
///
/// Обнуление баланса гарантирует успешное сообщение из `Drop`.
pub fn destroy_account(mut account: Account, target: &mut Account) {
    target.balance += account.balance;
    account.balance = 0;
}

/// Начисляет проценты на все аккаунты банка.
///
/// Ставки указаны в базисных пунктах: 1 пункт равен 0,01%.
///
/// ```
/// use lesson_09_ownership_borrowing_practice::{Account, Bank, accrue_interest};
///
/// let mut bank = Bank::new(vec![Account::new("Вклад", 10_000)], 100, 200);
/// accrue_interest(&mut bank);
/// assert_eq!(bank.accounts[0].balance, 10_100);
/// # bank.accounts[0].balance = 0;
/// ```
pub fn accrue_interest(bank: &mut Bank) {
    for account in &mut bank.accounts {
        let rate = if account.balance >= 0 {
            bank.credit_rate
        } else {
            bank.debit_rate
        };
        let interest = account.balance * i64::from(rate) / 10_000;
        account.balance += interest;
    }
}

/// Возвращает `(liabilities, assets)`: положительные остатки и модуль отрицательных.
pub fn bank_balance(bank: &Bank) -> (i64, i64) {
    let mut liabilities = 0_i64;
    let mut assets = 0_i64;

    for account in &bank.accounts {
        if account.balance >= 0 {
            liabilities += account.balance;
        } else {
            assets -= account.balance;
        }
    }

    (liabilities, assets)
}

/// Перемещает все аккаунты из `source` в `destination`.
pub fn merge_banks(source: Bank, destination: &mut Bank) {
    destination.accounts.extend(source.accounts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_funds_changes_both_accounts() {
        let mut from = Account::new("From", 500);
        let mut to = Account::new("To", 100);

        transfer_funds(&mut from, &mut to, 200);

        assert_eq!(from.balance, 300);
        assert_eq!(to.balance, 300);
        from.balance = 0;
        to.balance = 0;
    }

    #[test]
    fn destroy_account_transfers_remainder() {
        let account = Account::new("Closing", -150);
        let mut target = Account::new("Target", 500);

        destroy_account(account, &mut target);

        assert_eq!(target.balance, 350);
        target.balance = 0;
    }

    #[test]
    fn interest_uses_credit_and_debit_rates() {
        let mut bank = Bank::new(
            vec![
                Account::new("Deposit", 10_000),
                Account::new("Loan", -10_000),
            ],
            100,
            200,
        );

        accrue_interest(&mut bank);

        assert_eq!(bank.accounts[0].balance, 10_100);
        assert_eq!(bank.accounts[1].balance, -10_200);
        bank.accounts[0].balance = 0;
        bank.accounts[1].balance = 0;
    }

    #[test]
    fn bank_balance_separates_liabilities_and_assets() {
        let bank = Bank::new(
            vec![
                Account::new("A", 700),
                Account::new("B", -200),
                Account::new("C", 300),
                Account::new("D", -500),
            ],
            0,
            0,
        );

        assert_eq!(bank_balance(&bank), (1_000, 700));
    }

    #[test]
    fn merge_banks_moves_accounts() {
        let source = Bank::new(vec![Account::new("A", 0), Account::new("B", 0)], 10, 20);
        let mut destination = Bank::new(vec![Account::new("C", 0)], 30, 40);

        merge_banks(source, &mut destination);

        assert_eq!(destination.accounts.len(), 3);
        assert_eq!(destination.credit_rate, 30);
        assert_eq!(destination.debit_rate, 40);
    }
}
