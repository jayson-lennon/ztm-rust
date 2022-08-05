// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

/// A Student
struct Student {
    /// student's name
    name: String,
    /// student's locker number, if any
    locker: Option<i32>,
}

impl Student {
    /// print Student name and locker number
    fn print(&self) {
        match self.locker {
            Some(num) => println!("Student {:?}, locker number {:?}", self.name, num),
            None => println!("Student {:?}, no locker assigned", self.name),
        }
    }
}

fn main() {
    let student1 = Student {
        name: String::from("test"),
        locker: None,
    };
    let student2 = Student {
        name: String::from("r1oga"),
        locker: Some(10),
    };

    student1.print();
    student2.print();
}
