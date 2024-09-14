#![allow(dead_code)]

fn main() {}

#[derive(Clone, Copy, Debug, PartialEq)]
enum AccountEvent {
    Deposit(f64),
    Withdraw(f64),
}

#[derive(Debug, Default)]
struct Account {
    balance: f64,
    events: Vec<AccountEvent>,
}
impl Account {
    pub fn apply(&mut self, event: AccountEvent) -> f64 {
        match event {
            AccountEvent::Deposit(amount) => self.balance += amount,
            AccountEvent::Withdraw(amount) => self.balance -= amount,
        }
        self.events.push(event);
        self.balance
    }

    fn events(&self) -> &[AccountEvent] {
        &self.events
    }
}

impl FromIterator<AccountEvent> for Account {
    fn from_iter<T: IntoIterator<Item = AccountEvent>>(iter: T) -> Self {
        let mut account = Account::default();
        for event in iter {
            account.apply(event);
        }
        account
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait AccountTestExt {
        fn deposit(&mut self, amount: f64) -> f64;
        fn withdraw(&mut self, amount: f64) -> f64;
    }

    impl AccountTestExt for Account {
        fn deposit(&mut self, amount: f64) -> f64 {
            self.apply(AccountEvent::Deposit(amount))
        }

        fn withdraw(&mut self, amount: f64) -> f64 {
            self.apply(AccountEvent::Withdraw(amount))
        }
    }

    fn event_log(values: &[f64]) -> Vec<AccountEvent> {
        values
            .iter()
            .map(|amount| {
                let amount = *amount;
                if amount >= 0.0 {
                    AccountEvent::Deposit(amount)
                } else {
                    AccountEvent::Withdraw(-amount)
                }
            })
            .collect()
    }

    #[test]
    fn default_account_is_empty() {
        let account = Account::default();
        assert_eq!(account.balance, 0.0);
    }

    #[test]
    fn deposit_event_increases_account_balance() {
        // Given a default (empty) account
        let mut account = Account::default();

        // When making a deposit
        let updated_balance = account.deposit(10.0);

        // Then the balance increases by the deposit amount
        assert_eq!(updated_balance, 10.0);
    }

    #[test]
    fn withdraw_event_decreases_account_balance() {
        // Given an account having 10 units of currency
        let mut account = Account::default();
        account.deposit(10.0);

        // When making a withdraw
        let updated_balance = account.withdraw(2.0);

        // Then the balance decreases by the withdraw amount
        assert_eq!(updated_balance, 8.0);
    }

    #[test]
    fn account_audit_produces_all_events() {
        // Given an account with some events applied
        let mut account = Account::default();
        account.deposit(10.0);
        account.withdraw(2.0);

        // When we request the event log
        let events = account.events();

        // Then all events are returned
        let expected = event_log(&[10.0, -2.0]);
        assert_eq!(expected, events);
    }

    #[test]
    fn account_audit_produces_no_events_for_empty_account() {
        // Given a default account
        let account = Account::default();

        // When we request the event log
        let events = account.events();

        // Then no events are returned
        let expected = event_log(&[]);
        assert_eq!(expected, events);
    }

    #[test]
    fn account_is_created_from_empty_event_log() {
        // Given an empty event log
        let events = event_log(&[]);

        // When an account is created
        let account = Account::from_iter(events);

        // Then the account event log is empty
        assert!(account.events().is_empty());
    }

    #[test]
    fn account_is_created_from_event_log_containing_events() {
        // Given an event log
        let events = event_log(&[10.0, 2.0]);

        // When an account is created
        let account = Account::from_iter(events.clone());

        // Then the account event log contains the original events
        assert_eq!(events, account.events());
    }

    #[test]
    fn account_created_from_event_log_has_correct_balance() {
        // Given an event log
        let events = event_log(&[10.0, -2.0]);

        // When an account is created
        let account = Account::from_iter(events.clone());

        // Then the account has the correct balannce
        assert_eq!(account.balance, 8.0);
    }
}
