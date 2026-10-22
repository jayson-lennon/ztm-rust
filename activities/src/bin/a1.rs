// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn display_name(first: &str, last: &str) {
    println!("My name is {first} {last}");
}
fn main() {
    display_name("Jan", "Dy");
}
