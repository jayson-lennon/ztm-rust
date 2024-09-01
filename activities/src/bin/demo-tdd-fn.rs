fn main() {}

fn add(lhs: u32, rhs: u32) -> u32 {
    lhs + rhs
}

fn sub(lhs: i32, rhs: i32) -> i32 {
    lhs - rhs
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // test name describes:
    // 1. what we are doing
    // 2. what we are expecting
    #[test]
    fn add_function_adds_two_numbers() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn subtract_function_subtracts_two_numbers() {
        let result = sub(10, 2);
        assert_eq!(result, 8);
    }

    #[test]
    fn subtract_function_produces_a_negative_result_when_subtracting_a_large_number() {
        let result = sub(2, 10);
        assert_eq!(result, -8);
    }
}
