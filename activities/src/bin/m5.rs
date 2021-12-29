// Topic: Macro practice
//
// Summary:
//   Create a macro that can be used to generate new test cases for
//   the function provided.
//
// Requirements:
// * Write a macro to generate tests for `sample_fn`
// * Create at least 6 test cases using the macro
//   * Test the minimum and maximum values for each match arm
// * All test functions must be created by invoking the macro
//
// Notes:
// * Tuples can be used to specify both the input and expected output
// * The macro can be invoked multiple times; repetitions are optional

#[derive(Debug, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

fn sample_fn(n: u8) -> Size {
    use Size::*;
    match n {
        0..=53 => Small,
        54..=154 => Medium,
        155.. => Large
    }
}

fn main() {
    // use `cargo test --bin m5` to check your work
}

#[cfg(test)]
mod test { }
