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
    temp_readings: Vec<f64>,
}

impl TemperatureSensor {
    pub fn record_temperature(&mut self, temperature: f64) {
        self.temp_readings.push(temperature);
    }
    pub fn get_average_temperature(&self) -> f64 {
        self.temp_readings.iter().sum::<f64>() / self.temp_readings.len() as f64
    }
    pub fn get_max_temperature(&self) -> Option<f64> {
        if self.temp_readings.iter().len() == 0 {
            return None;
        }

        let mut max = f64::MIN;
        for tmp in self.temp_readings.iter() {
            if *tmp > max {
                max = *tmp
            }
        }

        Some(max)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_temperature_readings_list_is_empty() {
        let sensor = TemperatureSensor::default();
        assert_eq!(sensor.temp_readings.len(), 0);
    }
    #[test]
    fn add_a_new_temperature_reading() {
        let mut sensor = TemperatureSensor::default();
        sensor.record_temperature(33.0);

        assert_eq!(sensor.temp_readings.len(), 1);
    }
    #[test]
    fn calc_average_temperature() {
        let mut sensor = TemperatureSensor::default();
        sensor.record_temperature(23.0);
        sensor.record_temperature(37.0);
        sensor.record_temperature(39.0);
        let avg = sensor.get_average_temperature();

        assert_eq!(33.0, avg);
    }
    #[test]
    fn calc_max_temperature_on_non_empty_list() {
        let mut sensor = TemperatureSensor::default();
        sensor.record_temperature(23.0);
        sensor.record_temperature(37.0);
        sensor.record_temperature(39.0);
        let max = sensor.get_max_temperature();

        assert_eq!(39.0, max.unwrap());
    }
    #[test]
    fn calc_max_temperature_on_empty_list() {
        let sensor = TemperatureSensor::default();

        let max = sensor.get_max_temperature();

        assert_eq!(None, max);
    }
}
