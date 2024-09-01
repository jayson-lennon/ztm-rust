// Topic: Extension traits
//
// Summary:
//   The following program simulates an account management system where users can deposit and
//   withdraw money. The goal is to extend basic account operations with additional features using
//   an extension trait.
//
// Requirements:
// - Create an extension trait named `AccountExt` that adds two methods to the `Account` trait:
//   - `withdraw`: removes a specified amount from the account.
//   - `deposit`: adds a specified amount to the account.
// - Implement the `AccountExtensions` trait for any type that implements the `Account` trait by
//   using a blanket implementation.
// - Do not change any of the existing code. Only add and implement an extension trait.
//
// Expected Output:
//   Adjusted balance by $50.00. New balance: $150.00
//   Adjusted balance by -$30.00. New balance: $120.00
//   Adjusted balance by $20.00. New balance: $140.00

/**********************************************
* Do not change
**********************************************/
trait Account {
    fn adjust(&mut self, amount: f64);
}

struct BankAccount {
    balance: f64,
}

/**********************************************
* Do not change
**********************************************/
impl BankAccount {
    fn new(initial_balance: f64) -> Self {
        BankAccount {
            balance: initial_balance,
        }
    }
}

/**********************************************
* Do not change
**********************************************/
impl Account for BankAccount {
    fn adjust(&mut self, amount: f64) {
        self.balance += amount;
        println!(
            "Adjusted balance by ${:.2}. New balance: ${:.2}",
            amount, self.balance
        );
    }
}

/**********************************************
* Do not change
**********************************************/
fn main() {
    let mut account = BankAccount::new(100.0);

    // Using the basic process method to deposit money
    account.adjust(50.0);

    // Using the extended withdraw method to withdraw money
    account.withdraw(30.0);

    // Using the extended deposit method to deposit money
    account.deposit(20.0);
}
