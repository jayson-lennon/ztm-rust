use crate::bill::input::capture_input;
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
        use crate::bill::input::capture_input;

        let name = capture_input("Enter Name:").ok_or("");
        let status = capture_input("Enter Status:").ok_or("");

        let amount = capture_input("Enter Amount:").ok_or("");

        let mut bill = Bill {
            amount: amount.unwrap().parse::<i32>().unwrap(),
            name: name.unwrap(),
            status: match status.unwrap().to_lowercase().as_str() {
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
    counter: i32,
}

impl Bills {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add(&mut self) {
        self.counter += 1;
        let bill = Bill::new(&self.counter).unwrap();
        self.inner.insert(self.counter, bill);
    }

    pub fn remove(&mut self) {
        if self.inner.len() == 0 {
            println!("0 bills");
        }

        let id = capture_input("Bill id:").unwrap().parse::<i32>().unwrap();
        self.inner.remove(&id);
    }

    pub fn edit(&mut self) {
        if self.inner.len() == 0 {
            println!("0 bills");
        }

        let id = capture_input("Bill id:").unwrap().parse::<i32>().unwrap();
        println!("New bill attributes:");
        if let Some(bill) = self.inner.get_mut(&id) {
            *bill = Bill::new(&id).unwrap();
        }
    }

    pub fn view(&self) {
        if self.inner.len() == 0 {
            println!("0 bills")
        }

        for bill in self.inner.values() {
            bill.print()
        }
    }
}
