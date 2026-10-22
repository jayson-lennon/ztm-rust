// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(num_1: i32, num_2: i32) -> i32 {
    return num_1 + num_2;
}

fn display_result(result: i32) {
    println!("Result is {result}")
}
fn main() {
    let result: i32 = add(233, 23423434);
    display_result(result);
}
