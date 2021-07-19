// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;

/// A bill with a name and amount owed.
#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

/// Collection used to store bills.
pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    /// Create a new bills collection.
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Add a new bill. If a bill with the same name exists, it is overwritten.
    fn add(&mut self, bill: Bill) {
        // We need to clone the bill name, since the String type cannot be implicitly copied.
        // Without the clone, the name would get moved into the 'key' portion of the hashmap,
        // and therefore would be moved out of the bill struct.
        self.inner.insert(bill.name.clone(), bill);
    }

    /// Retrieve all the bills.
    fn get_all(&self) -> Vec<&Bill> {
        // The values function on HashMap will iterate over references
        // of the Bills, so we can just collect those directly into
        // a Vector.
        self.inner.values().collect()
    }

    /// Removes an existing bill. Returns false if the bill does not exist.
    fn remove(&mut self, name: &str) -> bool {
        // Chaning the is_some() function call will allow us to return
        // whether an item was removed or not.
        self.inner.remove(name).is_some()
    }

    /// Updates an existing bill. Returns false if the bill does not exist.
    fn update(&mut self, name: &str, amount: f64) -> bool {
        // We use the get_mut() function defined on the HashMap type
        // in order to change items present within the hashmap.
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

/// Retrieves user input. This function will automatically retry on
/// io errors, and will return None if the user did not enter any data.
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

/// Retrieves a bill amount. None is returned if the user did not
/// make any entry, otherwise will retry until the user enters an amount.
fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};

    /// Process for adding a new bill. Includes accepting user input
    /// and aborting if the user does not enter any data.
    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name:");
        // We can also use the question mark operator here
        // if we change the return type to an Option.
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added");
    }

    /// Process for removing an existing bill. Includes accepting user
    /// input and aborting if the user does not enter any data.
    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        if bills.remove(&name) {
            println!("removed");
        } else {
            println!("bill not found");
        }
    }

    /// Process for updating an existing bill. Includes accepting user
    /// input and aborting if the user does not enter any data.
    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill to update:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        if bills.update(&name, amount) {
            println!("updated");
        } else {
            println!("bill not found");
        }
    }

    /// Process for viewing existing bills.
    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }
}

// Enumeration over possible main menu options.
enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    // Try to convert a string into a MainMenu.
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }

    // Show the main menu so the user knows which
    // options are available.
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("");
        println!("Enter selection:");
    }
}

/// Main menu loop.
///
/// Displays the main menu and allows the user to make a selection.
/// Any entry that does not exist will abort the program.
fn run_main_menu() -> Option<String> {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => break,
        }
    }
    None
}

fn main() {
    run_main_menu();
}
