// Topic: Macro practice
//
// Summary:
//   Create a macro that can log when a closure is called, and how long
//   it takes to execute.
//
// Requirements:
// * Write a single macro that executes a closure:
//   * Prior to executing the closure, print out "Call: ", followed
//     by the closure name
//   * Track how long the closure takes to executes
//   * Print out the time taken in nanoseconds once execution completes
// * Call each sample function by wrapping each in a closure and
//   invoking the macro
//
// Notes:
// * Use `std::time::Instant` to calculate how long the closure takes to execute
// * Use `stringify!` to get a string representation of the closure name

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
