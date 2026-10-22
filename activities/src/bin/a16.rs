// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Assignment {
    name: String,
    locker: Option<i32>,
}
fn main() {
    let ass1: Assignment = Assignment {
        name: "Jan".to_owned(),
        locker: Some(100),
    };
    let ass2: Assignment = Assignment {
        name: "Adam".to_owned(),
        locker: None,
    };

    println!("Students name {}", ass1.name);
    match ass1.locker {
        Some(number) => println!("Assigment results are {number}"),
        None => println!("Did not provide"),
    }

    println!("Students name {}", ass2.name);
    match ass2.locker {
        Some(number) => println!("Assigment results are {number}"),
        None => println!("Did not provide"),
    }
}
