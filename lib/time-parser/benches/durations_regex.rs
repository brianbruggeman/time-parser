use std::time::Duration;
use chrono::Duration as ChronoDuration;
use regex::Regex;
use time_parser::{Error, Result};
use once_cell::sync::Lazy;

static HMS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^(\d+):(\d+):(\d+)$"#).expect("Invalid HMS pattern")
});

static SHORTHAND_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d+(?:_\d{3})*)([wdhms])").expect("Invalid shorthand pattern")
});

pub fn parse_duration_hms(interval: impl AsRef<str>) -> Result<Duration> {
    let interval = interval.as_ref();
    if let Some(captures) = HMS_PATTERN.captures(interval) {
        let hours: u64 = captures
            .get(1)
            .ok_or_else(|| Error::invalid_duration(interval))?
            .as_str()
            .parse::<u64>()
            .map_err(|_e| Error::invalid_time(interval))?;
        let minutes: u64 = captures
            .get(2)
            .ok_or_else(|| Error::invalid_duration(interval))?
            .as_str()
            .parse::<u64>()
            .map_err(|_e| Error::invalid_time(interval))?;
        let seconds: u64 = captures
            .get(3)
            .ok_or_else(|| Error::invalid_duration(interval))?
            .as_str()
            .parse::<u64>()
            .map_err(|_e| Error::invalid_time(interval))?;

        Ok(Duration::from_secs(hours * 3600 + minutes * 60 + seconds))
    } else {
        Err(Error::invalid_duration(interval))
    }
}

pub fn parse_duration_shorthand(interval: impl AsRef<str>) -> Result<Duration> {
    let interval = interval.as_ref();
    let mut delta: Option<ChronoDuration> = None;
    let mut has_valid_unit = false;

    for cap in SHORTHAND_PATTERN.captures_iter(&interval.to_lowercase()) {
        let base_number_str = cap[1].replace('_', "");
        let base_number = base_number_str
            .parse::<i64>()
            .map_err(|_e| Error::invalid_duration(&cap[1]))?;

        let unit = &cap[2];
        has_valid_unit = true;

        let duration = match unit {
            "w" => ChronoDuration::weeks(base_number),
            "d" => ChronoDuration::days(base_number),
            "h" => ChronoDuration::hours(base_number),
            "m" => ChronoDuration::minutes(base_number),
            "s" => ChronoDuration::seconds(base_number),
            _ => return Err(Error::invalid_unit(unit)),
        };

        delta = Some(delta.map_or(duration, |d| d + duration));
    }

    if !has_valid_unit || delta.is_none() {
        return Err(Error::invalid_duration(interval));
    }

    if let Some(total_duration) = delta {
        let total_seconds = total_duration.num_seconds();
        Ok(Duration::from_secs(total_seconds as u64))
    } else {
        Err(Error::invalid_duration(interval))
    }
}

pub fn parse_duration(interval: impl AsRef<str>) -> Result<Duration> {
    parse_duration_hms(interval.as_ref()).or_else(|_| parse_duration_shorthand(interval))
}
