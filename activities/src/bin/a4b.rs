// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn display(val: i32) -> &'static str {
    return match val {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        _ => "other"
    }
}
fn main() {
    println!("{:?}", display(0));
    println!("{:?}", display(1));
    println!("{:?}", display(12));
    println!("{:?}", display(2));
    println!("{:?}", display(3))
}
