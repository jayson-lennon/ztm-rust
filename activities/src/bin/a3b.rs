// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn display(val: i32) -> &'static str {
    return if val > 5 {
        ">5"
    } else if val < 5 {
        "< 5"
    } else {
        "=5"
    }
}

fn main() {
    println!("{:?}", display(6));
    println!("{:?}", display(4));
    println!("{:?}", display(5))
}