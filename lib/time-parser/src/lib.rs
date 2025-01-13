mod constants;
mod duration_formatter;
mod durations;
mod error;

pub use constants::{NANOSECONDS_PER_MICROSECOND, NANOSECONDS_PER_MILLISECOND, SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, SECONDS_PER_WEEK};
pub use duration_formatter::DurationFormatter;
pub use durations::{duration_as_string, parse_duration, parse_duration_hms, parse_duration_shorthand, parse_durations};
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
