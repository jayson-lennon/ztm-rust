// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    color: String,
    name: String,
}

impl Person {
    fn print(&self) {
        println!("{:?} {:?}", self.name, self.color)
    }
}

fn main() {
    let persons = vec![
        Person {
            age: 5,
            color: String::from("blue"),
            name: String::from("albert"),
        },
        Person {
            age: 6,
            color: String::from("yellow"),
            name: String::from("paul"),
        },
        Person {
            age: 5,
            color: String::from("red"),
            name: String::from("john"),
        },
        Person {
            age: 12,
            color: String::from("green"),
            name: String::from("fanny"),
        },
    ];

    for person in persons {
        if person.age < 10 {
            person.print()
        }
    }
}
