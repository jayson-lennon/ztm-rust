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
use activity::bill::*;

fn choice_one(bills: &Vec<Bill>) {
    println!("=====");
    println!("Viewing bills");

    if bills.len() == 0 {
        println!("0 bills")
    }

    for bill in bills {
        println!("{:?}", bill)
    }
}

fn choice_two(mut bills: Vec<Bill>) -> Vec<Bill> {
    println!("=====");
    println!("Creating a new bill");

    let bill = Bill::new();
    bills.push(bill);

    return bills;
}

fn main() {
    // use activity::bill::bill::Bill;
    // let bill = Bill::new();
    //
    // println!("{:?}", bill)

    let mut bills: Vec<Bill> = Vec::new();

    println!("MENU CLI");
    println!("=====");
    loop {
        for line in vec!["1: View bills", "2: Create bill", "q: quit"] {
            println!("{:?}", line)
        }
        println!("=====");

        let choice: String = input::capture_input("Make a choice: (1/2/q)");

        match choice.as_str() {
            "1" => choice_one(&bills),
            "2" => {
                let new_bills = choice_two(bills.clone());
                bills = new_bills
            }
            "q" => break,
            _ => println!("invalid choice, try again"),
        }

        println!("=====");
    }

    println!("done");
}
