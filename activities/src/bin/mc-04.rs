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

#[derive(Debug, Default)]
struct TemperatureSensor {
    readings: Vec<f64>,
}

impl TemperatureSensor {
    pub fn is_empty(&self) -> bool {
        self.readings.is_empty()
    }

    fn record_temperature(&mut self, temp: f64) {
        self.readings.push(temp);
    }

    fn get_average_temperature(&self) -> Option<f64> {
        (!self.readings.is_empty())
            .then_some(self.readings.iter().copied().sum::<f64>() / self.readings.len() as f64)
    }

    fn get_max_temperature(&self) -> Option<f64> {
        (!self.readings.is_empty()).then_some(
            self.readings
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, |a, b| a.max(b)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait TemperatureSensorTestExt {
        fn with_temps(temps: &[f64]) -> TemperatureSensor;
    }

    impl TemperatureSensorTestExt for TemperatureSensor {
        fn with_temps(temps: &[f64]) -> TemperatureSensor {
            let mut sensor = TemperatureSensor::default();
            for temp in temps {
                sensor.record_temperature(*temp);
            }
            sensor
        }
    }

    #[test]
    fn new_temperature_sensor_is_empty() {
        let sensor = TemperatureSensor::default();
        assert!(sensor.is_empty());
    }

    #[test]
    fn records_a_temperature() {
        let mut sensor = TemperatureSensor::default();

        sensor.record_temperature(30.0);

        assert!(!sensor.is_empty());
    }

    #[test]
    fn returns_average_temperature_with_no_readings() {
        let sensor = TemperatureSensor::default();

        let avg = sensor.get_average_temperature();

        assert!(avg.is_none());
    }

    #[test]
    fn returns_average_temperature_with_one_reading() {
        let sensor = TemperatureSensor::with_temps(&[10.0]);

        let avg = sensor.get_average_temperature();

        assert_eq!(avg, Some(10.0));
    }

    #[test]
    fn returns_average_temperature_with_two_readings() {
        let sensor = TemperatureSensor::with_temps(&[10.0, 20.0]);

        let avg = sensor.get_average_temperature();

        assert_eq!(avg, Some(15.0));
    }

    #[test]
    fn max_temp_returns_none_when_no_readings_are_present() {
        let sensor = TemperatureSensor::default();

        let max = sensor.get_max_temperature();

        assert!(max.is_none());
    }

    #[test]
    fn returns_max_temperature_with_two_readings() {
        let sensor = TemperatureSensor::with_temps(&[20.0, 10.0]);

        let max = sensor.get_max_temperature();

        assert_eq!(max, Some(20.0));
    }
}
