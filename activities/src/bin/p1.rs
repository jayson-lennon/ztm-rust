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

use activity::bill::bill::Bill;
use activity::bill::input::capture_input;
use activity::bill::utils::divider;
use std::collections::HashMap;

fn choice_one(bills: &HashMap<i32, Bill>) {
    divider(None, None);
    println!("Viewing bills");
    println!();

    if bills.len() == 0 {
        println!("0 bills")
    }

    for bill in bills {
        println!("{:?}", bill)
    }
}

fn choice_two(mut bills: HashMap<i32, Bill>) -> HashMap<i32, Bill> {
    divider(None, None);
    println!("Creating a new bill");
    println!();

    let bill = Bill::new();
    bills.insert(bill.id, bill);

    return bills;
}

fn choice_three(mut bills: HashMap<i32, Bill>) -> HashMap<i32, Bill> {
    divider(None, None);
    println!("Deleting existing bill");
    println!();

    if bills.len() == 0 {
        println!("0 bills");
        return bills;
    }

    let id = capture_input("Bill id:").parse::<i32>().unwrap();
    bills.remove(&id);

    return bills;
}

fn choice_four(mut bills: HashMap<i32, Bill>) -> HashMap<i32, Bill> {
    divider(None, None);
    println!("Editing existing bill");
    println!();

    if bills.len() == 0 {
        println!("0 bills");
        return bills;
    }

    let id = capture_input("Bill id:").parse::<i32>().unwrap();
    println!("New bill attributes:");
    if let Some(bill) = bills.get_mut(&id) {
        *bill = Bill::new();
    }

    return bills;
}

fn cli() {
    let mut bills: HashMap<i32, Bill> = HashMap::new();
    let mut prev_bills: HashMap<i32, Bill> = HashMap::new();

    loop {
        divider(None, None);

        for line in vec![
            "1: View bills",
            "2: Create bill",
            "3: Delete bill",
            "4: Edit bill",
            "q: quit",
            "x: cancel previous",
        ] {
            println!("{:?}", line)
        }

        divider(None, None);

        let choice: String = capture_input("Make a choice: (1/2/3/4/q/x)");

        match choice.as_str() {
            "1" => choice_one(&bills),
            "2" => {
                let new_bills = choice_two(bills.clone());
                prev_bills = bills;
                bills = new_bills
            }
            "3" => {
                let new_bills = choice_three(bills.clone());
                prev_bills = bills;
                bills = new_bills;
            }
            "4" => {
                let new_bills = choice_four(bills.clone());
                prev_bills = bills;
                bills = new_bills
            }
            "x" => {
                divider(None, None);
                println!("Cancelling previous action");
                bills = prev_bills.clone();
            }
            "q" => break,
            _ => println!("Invalid choice, try again"),
        }
    }
}

fn main() {
    println!("MENU CLI");

    cli();

    divider(None, None);
    println!("Done");
    divider(None, None);
}
