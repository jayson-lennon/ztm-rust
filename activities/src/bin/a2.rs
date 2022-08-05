// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(a:i32, b:i32) -> i32 {
    return a + b
}

fn  display_add(a:i32) {
    println!("{:?}", a)

}

fn main() {
    let result = add(4,7);
    display_add(result)
}
