use crate::bill::input::{capture_amount, capture_input};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum BillStatus {
    Paid,
    Unpaid,
    Overdue,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Bill {
    amount: i32,
    pub id: i32,
    name: String,
    status: BillStatus,
}

impl Bill {
    pub fn new(id: &i32) -> Result<Self, ()> {
        let name = capture_input("Enter Name:").ok_or("").unwrap();
        let status = capture_input("Enter Status:").ok_or("").unwrap();
        let amount = capture_amount()?;

        let mut bill = Bill {
            amount,
            name,
            status: match status.to_lowercase().as_str() {
                "paid" => BillStatus::Paid,
                "unpaid" => BillStatus::Unpaid,
                "overdue" => BillStatus::Overdue,
                _ => BillStatus::Unknown,
            },
            id: id.to_owned(),
        };

        return Ok(bill);
    }

    fn print(&self) {
        println!("{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Bills {
    inner: HashMap<i32, Bill>,
    previous: HashMap<i32, Bill>,
    counter: i32,
}

impl Bills {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            previous: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add(&mut self) {
        self.previous = self.inner.clone();
        self.counter += 1;
        let bill = Bill::new(&self.counter).unwrap();
        self.inner.insert(self.counter, bill);
    }

    pub fn remove(&mut self) {
        if self.inner.len() == 0 {
            println!("0 bills");
            return;
        }

        self.previous = self.inner.clone();

        let id = capture_input("Bill id:").unwrap().parse::<i32>().unwrap();

        match self.inner.remove(&id).is_some() {
            true => println!("Bill removed"),
            false => println!("Bill not found"),
        }
    }

    pub fn edit(&mut self) {
        if self.inner.len() == 0 {
            println!("0 bills");
            return;
        }

        self.previous = self.inner.clone();

        let id = capture_input("Bill id:").unwrap().parse::<i32>().unwrap();
        match self.inner.get_mut(&id) {
            Some(bill) => {
                println!("New bill attributes:");
                *bill = Bill::new(&id).unwrap();
                println!("Bill updated")
            }
            None => println!("Bill not found"),
        }
    }

    pub fn view(&self) {
        if self.inner.len() == 0 {
            println!("0 bills");
            return;
        }

        for bill in self.inner.values() {
            bill.print()
        }
    }

    pub fn reset(&mut self) {
        self.inner = self.previous.clone();
    }
}
