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
    name: String,
    color: String,
}

fn display_name(name: &str) {
    println!("Name is {name}");
}

fn display_color(color: &str) {
    println!("Color is {color}");
}

fn main() {
    let my_vec: Vec<Person> = vec![
        Person {
            name: String::from("Jan"),
            age: 2,
            color: String::from("Red"),
        },
        Person {
            name: String::from("Adam"),
            age: 92,
            color: String::from("Green"),
        },
        Person {
            name: String::from("Ola"),
            age: 10,
            color: String::from("Blue"),
        },
    ];

    for person in my_vec {
        if person.age <= 10 {
            // display_color(&person.color);
            display_name(&person.name);
        }
    }
}
