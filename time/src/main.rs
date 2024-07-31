use time::measure_time;

// Use the #[measure_time] macro to measure function execution time
fn my_function() {
    // Function body, where code that needs to measure execution time can be placed
    for _ in 0..1_000_000 {
        // Simulate a time-consuming operation
        let _ = 2 * 2;
    }
}

fn main() {
    let _res = measure_time!(my_function(), "my_function");
}

mod tests {
    use super::*;
    use jiff::{Timestamp, ToSpan};

    #[test]
    fn test_measure_time() {
        let _res = measure_time!(my_function(), "my_function");
    }

    #[test]
    fn test_jiff() -> Result<(), jiff::Error> {
        let time: Timestamp = "2024-07-11T01:14:00Z".parse()?;
        let zoned = time
            .intz("America/New_York")?
            .checked_add(1.month().hours(2))?;
        assert_eq!(
            zoned.to_string(),
            "2024-08-10T23:14:00-04:00[America/New_York]"
        );
        // Or, if you want an RFC3339 formatted string:
        assert_eq!(zoned.timestamp().to_string(), "2024-08-11T03:14:00Z");
        Ok(())
    }
}
