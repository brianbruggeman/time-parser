
pub trait DurationFormatter {
    fn parse<S>(interval: S) -> crate::Result<Self>
    where
        S: AsRef<str>,
        Self: Sized;
    fn to_string(&self) -> String;
}

impl DurationFormatter for std::time::Duration {
    fn parse<S>(interval: S) -> crate::Result<Self>
    where
        S: AsRef<str>,
        Self: Sized,
    {
        crate::durations::parse_duration(interval)
    }

    fn to_string(&self) -> String {
        crate::durations::duration_as_string(self)
    }
}

impl DurationFormatter for chrono::Duration {
    fn parse<S>(interval: S) -> crate::Result<Self>
    where
        S: AsRef<str>,
        Self: Sized,
    {
        let std_duration = crate::durations::parse_duration(interval);
        match std_duration {
            Ok(duration) => Ok(Self::from_std(duration).unwrap()),
            Err(e) => Err(e),
        }
    }

    fn to_string(&self) -> String {
        let std_duration = chrono_to_std(*self);
        crate::durations::duration_as_string(&std_duration)
    }
}

fn chrono_to_std(chrono_duration: chrono::Duration) -> std::time::Duration {
    let seconds = chrono_duration.num_seconds();
    let nanos = chrono_duration.num_nanoseconds().unwrap_or(0);
    std::time::Duration::new(seconds as u64, nanos as u32)
}

#[cfg(test)]
mod tests {
    use std::time::Duration as StdDuration;

    use chrono::Duration as ChronoDuration;
    use rstest::rstest;

    use super::*;
    use crate::constants::*;
    use crate::{Error, Result};

    #[rstest]
    #[case::valid_shorthand_2d3h4m("2d3h4m", Ok(StdDuration::from_secs(SECONDS_PER_DAY * 2 + SECONDS_PER_HOUR * 3 + SECONDS_PER_MINUTE * 4)))]
    #[case::valid_shorthand_1w2d3h4m5s6ms7us8ns("1w2d3h4m5s6ms7us8ns", Ok(StdDuration::new(SECONDS_PER_WEEK + 2 * SECONDS_PER_DAY + 3 * SECONDS_PER_HOUR + 4 * SECONDS_PER_MINUTE + 5, (6 * NANOSECONDS_PER_MILLISECOND + 7 * NANOSECONDS_PER_MICROSECOND + 8) as u32)))]
    #[case::valid_shorthand_600ms("600ms", Ok(StdDuration::new(0, (600 * NANOSECONDS_PER_MILLISECOND) as u32)))]
    #[case::valid_hms_02_03_04("02:03:04", Ok(StdDuration::from_secs(SECONDS_PER_HOUR * 2 + SECONDS_PER_MINUTE * 3 + 4)))]
    #[case::invalid_version("1.2.3", Err(Error::invalid_duration("1.2.3")))]
    fn test_duration_formatter_std(#[case] interval: &str, #[case] expected: Result<StdDuration>) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::ERROR)
            .try_init()
            .ok();

        let result = StdDuration::parse(interval);
        match expected {
            Ok(expected) => match result {
                Ok(result) => assert_eq!(result, expected, "expected: {:?}, actual: {:?}", expected, result),
                Err(e) => panic!("Expected Ok, got Err: {:?}", e),
            },
            Err(e) => match result {
                Ok(actual) => panic!("Expected Err, got Ok {:?} - {:?}", actual, e),
                Err(result) => assert_eq!(result.to_string(), e.to_string()),
            },
        }
    }

    #[rstest]
    #[case::valid_shorthand_2d3h4m("2d3h4m", Ok(ChronoDuration::new((SECONDS_PER_DAY * 2 + SECONDS_PER_HOUR * 3 + SECONDS_PER_MINUTE * 4) as i64, 0)))]
    #[case::valid_shorthand_1w2d3h4m5s6ms7us8ns("1w2d3h4m5s6ms7us8ns", Ok(ChronoDuration::new((SECONDS_PER_WEEK + 2 * SECONDS_PER_DAY + 3 * SECONDS_PER_HOUR + 4 * SECONDS_PER_MINUTE + 5) as i64, (6 * NANOSECONDS_PER_MILLISECOND + 7 * NANOSECONDS_PER_MICROSECOND + 8) as u32)))]
    #[case::valid_shorthand_600ms("600ms", Ok(ChronoDuration::new(0, (600 * NANOSECONDS_PER_MILLISECOND) as u32)))]
    #[case::valid_hms_02_03_04("02:03:04", Ok(ChronoDuration::new((SECONDS_PER_HOUR * 2 + SECONDS_PER_MINUTE * 3 + 4) as i64, 0)))]
    #[case::invalid_version("1.2.3", Err(Error::invalid_duration("1.2.3")))]
    fn test_duration_formatter_chrono(#[case] interval: &str, #[case] expected: Result<Option<ChronoDuration>>) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::ERROR)
            .try_init()
            .ok();

        let result = ChronoDuration::parse(interval);
        match expected {
            Ok(Some(expected)) => match result {
                Ok(result) => assert_eq!(result, expected, "expected: {:?}, actual: {:?}", expected, result),
                Err(e) => panic!("Expected Ok, got Err: {:?}", e),
            },
            Ok(None) => match result {
                Ok(actual) => panic!("Expected Ok, got Ok {:?}", actual),
                Err(e) => panic!("Expected Err, got Err: {:?}", e),
            },
            Err(e) => match result {
                Ok(actual) => panic!("Expected Err, got Ok {:?} - {:?}", actual, e),
                Err(result) => assert_eq!(result.to_string(), e.to_string()),
            },
        }
    }
}
