use std::time::Duration;

use pest::Parser;
use pest_derive::Parser;

use crate::{Error, Result, NANOSECONDS_PER_MICROSECOND, NANOSECONDS_PER_MILLISECOND, SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, SECONDS_PER_WEEK};

#[derive(Parser)]
#[grammar = "grammars/intervals.pest"]
struct DurationParser;

#[inline(always)]
pub fn parse_duration_hms(interval: impl AsRef<str>) -> Result<Duration> {
    let parse_result = DurationParser::parse(Rule::duration_hms, interval.as_ref()).map_err(|_e| Error::invalid_duration(interval.as_ref()))?;

    let mut parsed = false;
    let mut seconds = 0;
    let mut minutes = 0;
    let mut hours = 0;

    for pair in parse_result {
        if pair.as_rule() == Rule::duration_hms {
            (hours, minutes, seconds, parsed) = parse_hms_inner(pair, interval.as_ref())?;
        } else {
            tracing::trace!("Unexpected top-level rule: {:?}", pair.as_rule());
        }
    }

    if !parsed {
        tracing::trace!("No valid hms values found in interval: {}", interval.as_ref());
        return Err(Error::invalid_duration(interval.as_ref()));
    }

    let total_seconds = hours * SECONDS_PER_HOUR + minutes * SECONDS_PER_MINUTE + seconds;

    Ok(Duration::from_secs(total_seconds))
}

#[inline(always)]
fn parse_hms_inner(pair: pest::iterators::Pair<Rule>, interval: &str) -> Result<(u64, u64, u64, bool)> {
    let mut parsed = false;
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::hms_hour => {
                hours = parse_hour(inner_pair, interval)?;
                parsed = true;
            }
            Rule::hms_minute => {
                minutes = parse_minute(inner_pair, interval)?;
            }
            Rule::hms_second => {
                seconds = parse_second(inner_pair, interval)?;
            }
            _ => {
                tracing::trace!("Unexpected rule: {:?}", inner_pair.as_rule());
            }
        }
    }

    Ok((hours, minutes, seconds, parsed))
}

#[inline(always)]
fn parse_hour(pair: pest::iterators::Pair<Rule>, interval: &str) -> Result<u64> {
    let value = pair.as_str().parse::<u64>().map_err(|_| {
        tracing::trace!("Invalid hour value: {}", pair.as_str());
        Error::invalid_duration(interval)
    })?;
    Ok(value)
}

#[inline(always)]
fn parse_minute(pair: pest::iterators::Pair<Rule>, interval: &str) -> Result<u64> {
    let value = pair.as_str().parse::<u64>().map_err(|_| {
        tracing::trace!("Invalid minute value: {}", pair.as_str());
        Error::invalid_duration(interval)
    })?;

    if value > 59 {
        tracing::trace!("Invalid minute value: {}", pair.as_str());
        return Err(Error::invalid_duration(interval));
    }
    Ok(value)
}

#[inline(always)]
fn parse_second(pair: pest::iterators::Pair<Rule>, interval: &str) -> Result<u64> {
    let value = pair.as_str().parse::<u64>().map_err(|_| {
        tracing::trace!("Invalid second value: {}", pair.as_str());
        Error::invalid_duration(interval)
    })?;

    if value > 59 {
        tracing::trace!("Invalid second value: {}", pair.as_str());
        return Err(Error::invalid_duration(interval));
    }
    Ok(value)
}

#[inline(always)]
pub fn parse_duration_shorthand(interval: impl AsRef<str>) -> Result<Duration> {
    let parse_result = DurationParser::parse(Rule::duration_shorthand, interval.as_ref()).map_err(|_e| Error::invalid_duration(interval.as_ref()))?;

    let mut delta_seconds = 0;
    let mut delta_nanoseconds: u64 = 0;

    for pair in parse_result {
        if pair.as_rule() == Rule::duration_shorthand {
            let (seconds, nanoseconds, parsed) = parse_shorthand_inner(pair, interval.as_ref())?;
            if parsed {
                delta_seconds += seconds;
                delta_nanoseconds += nanoseconds;
            }
        } else {
            tracing::trace!("Unexpected top-level rule: {:?}", pair.as_rule());
        }
    }

    Ok(Duration::new(delta_seconds, delta_nanoseconds as u32))
}

#[inline(always)]
fn parse_shorthand_inner(pair: pest::iterators::Pair<Rule>, interval: &str) -> Result<(u64, u64, bool)> {
    let mut parsed = false;
    let mut delta_seconds = 0;
    let mut delta_nanoseconds: u64 = 0;

    let mut inner_pairs = pair.into_inner().peekable();
    while let Some(inner_pair) = inner_pairs.next() {
        if inner_pair.as_rule() == Rule::shorthand_digits {
            let digits_str = inner_pair.as_str();
            let value = digits_str
                .replace('_', "")
                .parse::<u64>()
                .map_err(|_| Error::invalid_duration(interval))?;

            if let Some(unit_pair) = inner_pairs.next() {
                let (delta_sec, delta_nano) = match unit_pair.as_rule() {
                    Rule::units_weeks => (value * SECONDS_PER_WEEK, 0),
                    Rule::units_days => (value * SECONDS_PER_DAY, 0),
                    Rule::units_hours => (value * SECONDS_PER_HOUR, 0),
                    Rule::units_minutes => (value * SECONDS_PER_MINUTE, 0),
                    Rule::units_seconds => (value, 0),
                    Rule::units_milliseconds => (0, value * NANOSECONDS_PER_MILLISECOND),
                    Rule::units_microseconds => (0, value * NANOSECONDS_PER_MICROSECOND),
                    Rule::units_nanoseconds => (0, value),
                    _ => {
                        tracing::trace!("Unexpected unit: {:?}", unit_pair.as_str());
                        return Err(Error::invalid_duration(interval));
                    }
                };
                delta_seconds += delta_sec;
                delta_nanoseconds += delta_nano;
                parsed = true;
            } else {
                tracing::trace!("Missing unit for shorthand: {}", digits_str);
                return Err(Error::invalid_duration(interval));
            }
        } else {
            tracing::trace!("Unexpected rule: {:?}", inner_pair.as_rule());
        }
    }

    Ok((delta_seconds, delta_nanoseconds, parsed))
}

#[inline(always)]
pub fn parse_duration(interval: impl AsRef<str>) -> Result<Duration> {
    let parse_result = DurationParser::parse(Rule::duration, interval.as_ref()).map_err(|_e| Error::invalid_duration(interval.as_ref()))?;

    for pair in parse_result {
        if pair.as_rule() == Rule::duration {
            return parse_duration_inner(pair, interval.as_ref());
        }
    }

    tracing::trace!("No valid duration values found in input: {}", interval.as_ref());
    Err(Error::invalid_duration(interval.as_ref()))
}

#[inline(always)]
fn parse_duration_inner(pair: pest::iterators::Pair<Rule>, input: &str) -> Result<Duration> {
    let mut delta_seconds = 0;
    let mut delta_nanoseconds: u64 = 0;
    let mut parsed = false;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::duration_shorthand => {
                let (seconds, nanoseconds, _) = parse_shorthand_inner(inner_pair, input)?;
                delta_seconds += seconds;
                delta_nanoseconds += nanoseconds;
                parsed = true;
            }
            Rule::duration_hms => {
                let (hours, minutes, seconds, _) = parse_hms_inner(inner_pair, input)?;
                delta_seconds += hours * SECONDS_PER_HOUR + minutes * SECONDS_PER_MINUTE + seconds;
                parsed = true;
            }
            _ => unreachable!("Unexpected rule in duration parsing: {:?}", inner_pair.as_rule()),
        }
    }

    if !parsed {
        tracing::trace!("No valid duration values found in interval: {}", input);
        return Err(Error::invalid_duration(input));
    }

    Ok(Duration::new(delta_seconds, delta_nanoseconds as u32))
}

#[inline(always)]
pub fn parse_durations(string: impl AsRef<str>) -> Result<Vec<Duration>> {
    let parse_result = DurationParser::parse(Rule::durations, string.as_ref()).map_err(|_e| Error::invalid_duration(string.as_ref()))?;

    let mut durations = Vec::new();

    for pair in parse_result {
        if pair.as_rule() == Rule::duration {
            durations.push(parse_duration_inner(pair, string.as_ref())?);
        } else {
            unreachable!("Unexpected rule in durations parsing: {:?}", pair.as_rule());
        }
    }

    if durations.is_empty() {
        tracing::trace!("No valid duration values found in input: {}", string.as_ref());
        return Err(Error::invalid_duration(string.as_ref()));
    }

    Ok(durations)
}

#[inline(always)]
pub fn duration_as_string(duration: &Duration) -> String {
    let total_seconds = duration.as_secs();
    let nanos = duration.subsec_nanos() as u64;

    let weeks = total_seconds / SECONDS_PER_WEEK;
    let days = (total_seconds % SECONDS_PER_WEEK) / SECONDS_PER_DAY;
    let hours = (total_seconds % SECONDS_PER_DAY) / SECONDS_PER_HOUR;
    let minutes = (total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let seconds = total_seconds % SECONDS_PER_MINUTE;
    let milliseconds = nanos / NANOSECONDS_PER_MILLISECOND;
    let microseconds = (nanos % NANOSECONDS_PER_MILLISECOND) / NANOSECONDS_PER_MICROSECOND;
    let nanoseconds = nanos % NANOSECONDS_PER_MICROSECOND;

    let mut parts = Vec::new();
    if weeks > 0 {
        parts.push(format!("{}w", weeks));
    }
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if seconds > 0 {
        parts.push(format!("{}s", seconds));
    }
    if milliseconds > 0 {
        parts.push(format!("{}ms", milliseconds));
    }
    if microseconds > 0 {
        parts.push(format!("{}µs", microseconds));
    }
    if nanoseconds > 0 {
        parts.push(format!("{}ns", nanoseconds));
    }
    parts.join("")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::valid_shorthand_2d3h4m("2d3h4m", Ok(Duration::from_secs(SECONDS_PER_DAY * 2 + SECONDS_PER_HOUR * 3 + SECONDS_PER_MINUTE * 4)))]
    #[case::valid_shorthand_1w2d3h4m5s6ms7us8ns("1w2d3h4m5s6ms7us8ns", Ok(Duration::new(SECONDS_PER_WEEK + 2 * SECONDS_PER_DAY + 3 * SECONDS_PER_HOUR + 4 * SECONDS_PER_MINUTE + 5, (6 * NANOSECONDS_PER_MILLISECOND + 7 * NANOSECONDS_PER_MICROSECOND + 8) as u32)))]
    #[case::valid_shorthand_600ms("600ms", Ok(Duration::new(0, (600 * NANOSECONDS_PER_MILLISECOND) as u32)))]
    #[case::valid_hms_02_03_04("02:03:04", Ok(Duration::from_secs(SECONDS_PER_HOUR * 2 + SECONDS_PER_MINUTE * 3 + 4)))]
    #[case::invalid_version("1.2.3", Err(Error::invalid_duration("1.2.3")))]
    fn test_parse_duration(#[case] interval: &str, #[case] expected: Result<Duration>) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .try_init()
            .ok();

        let result = crate::durations::parse_duration(interval);
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
    #[case::valid_hms_00_00_00("00:00:00", Ok(Duration::from_secs(0)))]
    #[case::valid_hms_02_03_04("02:03:04", Ok(Duration::from_secs(SECONDS_PER_HOUR * 2 + SECONDS_PER_MINUTE * 3 + 4)))]
    #[case::invalid_hms_2d3h4m("2d3h4m", Err(Error::invalid_duration("2d3h4m")))]
    #[case::invalid_hms_230m("230m", Err(Error::invalid_duration("230m")))]
    #[case::invalid_version("1.2.3", Err(Error::invalid_duration("1.2.3")))]
    fn test_parse_hms_duration(#[case] interval: &str, #[case] expected: Result<Duration>) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .try_init()
            .ok();

        let result = parse_duration_hms(interval);
        match expected {
            Ok(expected) => match result {
                Ok(result) => assert_eq!(result, expected),
                Err(e) => panic!("Expected Ok, got Err: {:?}", e),
            },
            Err(e) => match result {
                Ok(actual) => panic!("Expected Err, got Ok {:?} - {:?}", actual, e),
                Err(result) => assert_eq!(result.to_string(), e.to_string()),
            },
        }
    }

    #[rstest]
    #[case::valid_shorthand_2d3h4m("2d3h4m", Ok(Duration::from_secs(SECONDS_PER_DAY * 2 + SECONDS_PER_HOUR * 3 + SECONDS_PER_MINUTE * 4)))]
    #[case::valid_shorthand_2h30m("2h30m", Ok(Duration::from_secs(SECONDS_PER_HOUR * 2 + SECONDS_PER_MINUTE * 30)))]
    #[case::valid_shorthand_230m("230m", Ok(Duration::from_secs(230 * SECONDS_PER_MINUTE)))]
    #[case::valid_shorthand_1d230m("1d230m", Ok(Duration::from_secs(SECONDS_PER_DAY + 230 * SECONDS_PER_MINUTE)))]
    #[case::valid_shorthand_1d230h("1d230h", Ok(Duration::from_secs(SECONDS_PER_DAY + 230 * SECONDS_PER_HOUR)))]
    #[case::valid_shorthand_1800000s("1800000s", Ok(Duration::from_secs(1800000)))]
    #[case::valid_shorthand_1_800_000s("1_800_000s", Ok(Duration::from_secs(1800000)))]
    #[case::valid_shorthand_1w7d("1w7d", Ok(Duration::from_secs(SECONDS_PER_WEEK + 7 * SECONDS_PER_DAY)))]
    #[case::valid_shorthand_1w1d("1w1d", Ok(Duration::from_secs(SECONDS_PER_WEEK + SECONDS_PER_DAY)))]
    #[case::valid_shorthand_1nanosecond("1nanosecond", Ok(Duration::from_nanos(1)))]
    #[case::valid_shorthand_1microsecond("1microsecond", Ok(Duration::from_micros(1)))]
    #[case::valid_shorthand_1millisecond("1millisecond", Ok(Duration::from_millis(1)))]
    #[case::valid_shorthand_1second("1second", Ok(Duration::from_secs(1)))]
    #[case::valid_shorthand_1minute("1minute", Ok(Duration::from_secs(SECONDS_PER_MINUTE)))]
    #[case::valid_shorthand_1hour("1hour", Ok(Duration::from_secs(SECONDS_PER_HOUR)))]
    #[case::valid_shorthand_1day("1day", Ok(Duration::from_secs(SECONDS_PER_DAY)))]
    #[case::valid_shorthand_1week("1week", Ok(Duration::from_secs(SECONDS_PER_WEEK)))]
    #[case::valid_shorthand_1_week("1 nanosecond", Ok(Duration::from_nanos(1)))]
    #[case::valid_shorthand_1_week("1 microsecond", Ok(Duration::from_micros(1)))]
    #[case::valid_shorthand_1_week("1 millisecond", Ok(Duration::from_millis(1)))]
    #[case::valid_shorthand_1_week("1 second", Ok(Duration::from_secs(1)))]
    #[case::valid_shorthand_1_week("1 minute", Ok(Duration::from_secs(SECONDS_PER_MINUTE)))]
    #[case::valid_shorthand_1_week("1 hour", Ok(Duration::from_secs(SECONDS_PER_HOUR)))]
    #[case::valid_shorthand_1_week("1 day", Ok(Duration::from_secs(SECONDS_PER_DAY)))]
    #[case::valid_shorthand_1_week("1 week", Ok(Duration::from_secs(SECONDS_PER_WEEK)))]
    #[case::valid_shorthand_8d("8d", Ok(Duration::from_secs(8 * SECONDS_PER_DAY)))]
    #[case::invalid_shorthand_1_day_caps("1 DAY", Err(Error::invalid_duration("1 DAY")))]
    #[case::invalid_shorthand_02_03_04("02:03:04", Err(Error::invalid_duration("02:03:04")))]
    #[case::invalid_version("1.2.3", Err(Error::invalid_duration("1.2.3")))]
    fn test_parse_shorthand_duration(#[case] interval: &str, #[case] expected: Result<Duration>) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .try_init()
            .ok();

        let result = parse_duration_shorthand(interval);
        match expected {
            Ok(expected) => match result {
                Ok(result) => assert_eq!(result, expected),
                Err(e) => panic!("Expected Ok, got Err: {:?}", e),
            },
            Err(e) => match result {
                Ok(actual) => panic!("Expected Err, got Ok {:?} - {:?}", actual, e),
                Err(result) => assert_eq!(result.to_string(), e.to_string()),
            },
        }
    }

    #[rstest]
    #[case::valid_shorthand_2d3h4m("2d3h4m", "2d3h4m")]
    #[case::valid_shorthand_2h30m("2h30m", "2h30m")]
    #[case::valid_shorthand_230m("230m", "3h50m")]
    #[case::valid_shorthand_1d230m("1d230m", "1d3h50m")]
    #[case::valid_shorthand_1d230h("1d230h", "1w3d14h")]
    #[case::valid_shorthand_1800000s("1800000s", "2w6d20h")]
    #[case::valid_shorthand_1_800_000s("1_800_000s", "2w6d20h")]
    #[case::valid_shorthand_1w7d("1w7d", "2w")]
    #[case::valid_shorthand_1w1d("1w1d", "1w1d")]
    #[case::valid_shorthand_8d("8d", "1w1d")]
    fn test_duration_as_string(#[case] interval: &str, #[case] expected: &str) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .try_init()
            .ok();

        let duration = crate::durations::parse_duration(interval).expect("interval must be valid for this test");
        let result = duration_as_string(&duration);
        assert_eq!(result, expected, "expected: {:?}, actual: {:?}", expected, result);
    }

    #[rstest]
    #[case::valid_durations_2d_and_3h("2d and 3h", &[Duration::from_secs(SECONDS_PER_DAY * 2), Duration::from_secs(SECONDS_PER_HOUR * 3)])]
    #[case::valid_durations_2d_and_3h_and_4m("2d and 3h and 4m", &[Duration::from_secs(SECONDS_PER_DAY * 2), Duration::from_secs(SECONDS_PER_HOUR * 3), Duration::from_secs(SECONDS_PER_MINUTE * 4)])]
    #[case::valid_durations_5d_and_01_02_03("5d and 01:02:03", &[Duration::from_secs(SECONDS_PER_DAY * 5), Duration::from_secs(SECONDS_PER_HOUR * 1 + SECONDS_PER_MINUTE * 2 + 3)])]
    fn test_parse_durations(#[case] interval: &str, #[case] expected: &[Duration]) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .try_init()
            .ok();

        let result = parse_durations(interval);
        match result {
            Ok(result) => assert_eq!(result, expected),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }
}
