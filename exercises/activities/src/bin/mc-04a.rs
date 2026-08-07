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
#[derive(Debug, Default)]
struct TemperatureSensor {
    readings: Vec<f64>,
}

impl TemperatureSensor {
    pub fn get_max_temp(&self) -> Option<f64> {
        (!self.readings.is_empty()).then_some(
            self.readings
                .iter()
                .fold(f64::NEG_INFINITY, |a, b| a.max(*b)),
        )
    }
}

impl TemperatureSensor {
    pub fn get_average_temp(&self) -> Option<f64> {
        (!self.readings.is_empty())
            .then_some(self.readings.iter().sum::<f64>() / self.readings.len() as f64)
    }
}

impl TemperatureSensor {
    pub fn is_empty(&self) -> bool {
        self.readings.is_empty()
    }

    pub fn record_temperature(&mut self, x: f64) {
        self.readings.push(x);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn returns_average_temp_with_no_readings() {
        let sensor = TemperatureSensor::default();
        let avg = sensor.get_average_temp();
        assert!(avg.is_none())
    }

    #[test]
    fn returns_average_temp_with_one_readings() {
        let mut sensor = TemperatureSensor::default();
        sensor.record_temperature(10.0);

        let avg = sensor.get_average_temp();
        assert_eq!(avg, Some(10.0));
    }

    #[test]
    fn returns_average_temp_with_two_readings() {
        let mut sensor = TemperatureSensor::default();
        sensor.record_temperature(10.0);
        sensor.record_temperature(40.0);

        let avg = sensor.get_average_temp();
        assert_eq!(avg, Some(25.0));
    }

    #[test]
    fn max_temp_returns_none_with_no_readings() {
        let sensor = TemperatureSensor::default();
        let max = sensor.get_max_temp();
        assert!(max.is_none())
    }

    #[test]
    fn returns_max_temp_with_two_readings() {
        let mut sensor = TemperatureSensor::default();
        sensor.record_temperature(10.0);
        sensor.record_temperature(40.0);

        let max = sensor.get_max_temp();
        assert_eq!(max, Some(40.0));
    }
}
