// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let mary = Student {
        name: "Mary".to_owned(),
        locker: Some(3),
    };

    println!("student: {:?}", mary.name);
    match mary.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assigned"),
    }
}
