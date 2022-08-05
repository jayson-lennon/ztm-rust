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
    id: i32,
    name: String,
    status: BillStatus,
}

impl Bill {
    pub fn new(id: i32) -> Self {
        use crate::bill::input::capture_input;

        let name: String = capture_input("Enter Name:");
        let status: String = capture_input("Enter Status:");

        let amount: String = capture_input("Enter Amount:");

        return Bill {
            amount: amount.parse::<i32>().unwrap(),
            id,
            name: name.to_owned(),
            status: match status.to_lowercase().as_str() {
                "paid" => BillStatus::Paid,
                "unpaid" => BillStatus::Unpaid,
                "overdue" => BillStatus::Overdue,
                _ => BillStatus::Unknown,
            },
        };
    }

    fn print(&self) {
        println!("{:?}", self)
    }
}
