#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        println!("Creating a new bank account with initial balance: {}", initial_balance);
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        println!("Depositing {} into the bank account", amount);
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from the bank account", amount);
        self.balance -= amount;
    }

    pub fn balance(&self) -> f64 {
        println!("Checking the balance of the bank account");
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(30.0);
        assert_eq!(account.balance(), 70.0);
    }

    // Add more tests here
}

/*
Hints
    Use assert_eq! to check if values are equal.
    Remember to test both normal operations and edge cases.
    For floating-point comparisons, you might want to use assert!((a - b).abs() < epsilon) where epsilon is a small number like 1e-10, to account for potential floating-point inaccuracies.
*/