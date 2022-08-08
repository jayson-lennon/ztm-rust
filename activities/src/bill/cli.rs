use crate::bill::bill::{Bill, Bills};
use crate::bill::input::capture_input;
use crate::bill::utils::divider;
use std::collections::HashMap;

enum Menu {
    AddBill,
    ViewBill,
    DeleteBill,
    EditBill,
    Quit,
    CancelPrevious,
}

impl Menu {
    fn from_str(input: &str) -> Option<Menu> {
        match input {
            "1" => Some(Self::ViewBill),
            "2" => Some(Self::AddBill),
            "3" => Some(Self::DeleteBill),
            "4" => Some(Self::EditBill),
            "q" => Some(Self::Quit),
            "x" => Some(Self::CancelPrevious),
            _ => None,
        }
    }

    fn show() {
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
    }
}

pub fn cli() {
    let mut bills = Bills::new();
    let mut prev_bills = Bills::new();

    loop {
        Menu::show();
        let choice = capture_input("Make a choice: (1/2/3/4/q/x)").expect("No data entered");
        divider(None, None);

        use Menu::*;
        match Menu::from_str(&choice) {
            Some(ViewBill) => {
                println!("Viewing bills");
                println!();

                bills.view();
            }
            Some(AddBill) => {
                prev_bills = bills.clone();
                println!("Creating a new bill");
                println!();

                bills.add();
            }
            Some(DeleteBill) => {
                prev_bills = bills.clone();
                println!("Deleting existing bill");
                println!();

                bills.remove();
            }
            Some(EditBill) => {
                println!("Editing existing bill");
                println!();

                bills.edit();
            }
            Some(CancelPrevious) => {
                println!("Cancelling previous action");
                bills.reset();
            }
            Some(Quit) => break,
            None => println!("Invalid choice, try again"),
        }
    }
}
