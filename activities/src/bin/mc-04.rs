// Topic: Test-driven development (TDD)
//
// Summary:
//   You have been tasked to create a system that records and analyzes readings from a temperature
//   sensor. Implement the program requirements using TDD.
//
// Requirements:
// - Define a `TemperatureSensor` struct that stores a list of temperature readings (e.g., as a `Vec<f64>`).
// - Implement these methods:
//    - `record_temperature`: adds a new temperature reading to the sensor's list
//    - `get_average_temperature`: returns the average temperature from the recorded readings
//    - `get_max_temperature`: returns the highest temperature recorded by the sensor
//
// Notes:
// - When using TDD, use the `red-green-refactor` method:
//    1. Red: Write _one_ test and then run it to make sure it fails
//    2. Green: Implement the code to make the test pass
//    3. Refactor: Clean up your implementation (if needed) and reduce duplication in your test cases
// - Feel free to add extra methods as needed (like to check if any temperatures have been recorded)
// - Use `cargo test --bin mc-04` to run your tests
// - The `.max()` method on iterators won't work for f64. Consider writing a `for` loop and
//   manually track the highest temperature, or use `.fold`

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature() {
        todo!()
    }
}
