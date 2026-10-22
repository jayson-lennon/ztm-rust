// Topic: Working with expressions
//
// Requirements:
// * Print "it's big" if a variable is > 100
// * Print "it's small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
fn print_mess(var: i32) {
    let is_big: bool = var > 100;
    match is_big {
        True => println!("Its big"),
        _ => println!("Its small"),
    }
}

fn main() {
    print_mess(343);
}
