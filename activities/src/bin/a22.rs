// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

extern crate core;

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn can_clamp() {
        assert_eq!(clamp(7, 8, 10), 8);
        assert_eq!(clamp(11, 8, 10), 10);
        assert_eq!(clamp(9, 8, 10), 9);
    }

    #[test]
    fn can_div() {
        assert_eq!(div(6, 3), Some(2));
    }

    #[test]
    fn can_concat() {
        assert_eq!(concat("a", "b"), "a b")
    }
}
