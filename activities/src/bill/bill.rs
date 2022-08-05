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
    pub fn new() -> Self {
        use crate::bill::input::capture_input;

        let name: String = capture_input("Enter Name:");
        let status: String = capture_input("Enter Status:");

        let amount: String = capture_input("Enter Amount:");

        let mut bill = Bill {
            amount: amount.parse::<i32>().unwrap(),
            name: name.to_owned(),
            status: match status.to_lowercase().as_str() {
                "paid" => BillStatus::Paid,
                "unpaid" => BillStatus::Unpaid,
                "overdue" => BillStatus::Overdue,
                _ => BillStatus::Unknown,
            },
            id: 0,
        };

        // address of pointer
        let id = &bill as *const Bill;
        bill.id = id as i32;

        return bill;
    }

    fn print(&self) {
        println!("{:?}", self)
    }
}
