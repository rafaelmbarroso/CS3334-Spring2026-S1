use crate::bank_account::BankAccount;
mod bank_account;

fn main() {
    let mut account = BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("Balance after deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("Balance after withdrawal: {}", account.balance());
}
