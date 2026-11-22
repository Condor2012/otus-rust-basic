use lesson_09_ownership_borrowing_practice::{
    Account, Bank, accrue_interest, bank_balance, destroy_account, merge_banks, print_balance,
    transfer_funds,
};

fn main() {
    let mut alice = Account::new("Alice", 1_000);
    let bob = Account::new("Bob", -200);

    print_balance(&alice);
    print_balance(&bob);

    transfer_funds(&mut alice, &mut Account::new("Temporary", 0), 100);
    print_balance(&alice);

    let mut main_bank = Bank::new(vec![alice, bob], 100, 200);
    accrue_interest(&mut main_bank);

    let (liabilities, assets) = bank_balance(&main_bank);
    println!("Обязательства банка: {liabilities}, активы: {assets}");

    let merged = Bank::new(vec![Account::new("Merged account", 0)], 50, 75);
    merge_banks(merged, &mut main_bank);
    println!("После объединения аккаунтов: {}", main_bank.accounts.len());

    let closing = Account::new("Closing", 300);
    destroy_account(closing, &mut main_bank.accounts[0]);
    print_balance(&main_bank.accounts[0]);

    // Аккаунты уничтожатся в обратном порядке при выходе main из области видимости.
}
