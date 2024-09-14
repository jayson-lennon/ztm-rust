#![allow(dead_code)]
#![allow(clippy::comparison_chain)]

fn main() {}

fn add(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}

fn max(lhs: i32, rhs: i32) -> Option<i32> {
    if lhs > rhs {
        Some(lhs)
    } else if lhs < rhs {
        Some(rhs)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_function_adds_two_numbers() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn add_function_adds_two_negative_numbers() {
        let result = add(-1, 4);
        assert_eq!(result, 3);
    }

    #[test]
    fn max_function_returns_first_number_when_it_is_greatest() {
        let result = max(5, 1);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn max_function_returns_second_number_when_it_is_greatest() {
        let result = max(2, 3);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn max_function_returns_none_when_both_numbers_are_equal() {
        let result = max(4, 4);
        assert!(result.is_none());
    }
}
