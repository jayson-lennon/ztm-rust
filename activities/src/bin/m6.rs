// Topic: Macro practice
//
// Summary:
//   Create a macro that measures how long a function takes to execute.
//
// Requirements:
// * Write a single macro that executes a function:
//   * Prior to executing the function, print out "Call: ", followed
//     by the function name
//   * Measure how long the function takes to executes
//   * Print out (in nanoseconds) how long the function takes to execute
// * Measure each sample function with the macro
//
// Notes:
// * `std::time::Instant` can be used to calculate elapsed time
// * Use `stringify!` to get a string representation of the function name

fn sample_fn_1() {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(2));
}
fn sample_fn_2(n: u64) {
    let mut n = n;
    while n > 0 {
        use std::time::Duration;
        std::thread::sleep(Duration::from_micros(n));
        n -= 1;
    }
}
fn sample_fn_3(lhs: usize, rhs: usize) -> usize {
    lhs + rhs
}

fn main() {}
