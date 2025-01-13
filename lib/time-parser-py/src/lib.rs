use std::time::Duration;

use pyo3::prelude::*;
use pyo3::types::{PyDelta, PyDeltaAccess};

mod durations {
    pub const NANOSECONDS_PER_MICROSECOND: u64 = 1_000;
    pub use time_parser::*;
}

#[pyfunction]
fn parse_timedelta(interval: &str) -> PyResult<PyObject> {
    Python::with_gil(|py: Python<'_>| {
        let datetime = PyModule::import(py, "datetime")?;
        let timedelta = datetime.getattr("timedelta")?;
        match durations::parse_duration(interval) {
            Ok(duration) => {
                // Import the `datetime` module and create a `timedelta` object
                const HOURS: i64 = 0;
                const MINUTES: i64 = 0;
                const WEEKS: i64 = 0;

                let total_seconds = duration.as_secs();
                let total_microseconds = duration.subsec_micros();
                let days: i64 = (total_seconds / durations::SECONDS_PER_DAY) as i64;
                let seconds: i64 = (total_seconds % durations::SECONDS_PER_DAY) as i64;
                let microseconds: i64 = total_microseconds as i64;

                let new_timedelta_object = timedelta.call1((days, seconds, microseconds, MINUTES, HOURS, WEEKS))?;
                Ok(new_timedelta_object.into())
            }
            Err(_) => Err(pyo3::exceptions::PyValueError::new_err("Invalid duration")),
        }
    })
}

#[pyfunction]
fn timedelta_to_string(timedelta: &Bound<'_, PyDelta>) -> PyResult<String> {
    let days = timedelta.get_days() as u64;
    let seconds = timedelta.get_seconds() as u64;
    let microseconds = timedelta.get_microseconds() as u64;

    let seconds = days * durations::SECONDS_PER_DAY + seconds;
    let nanoseconds = microseconds * durations::NANOSECONDS_PER_MICROSECOND;
    let duration = Duration::new(seconds, nanoseconds as u32);

    Ok(durations::duration_as_string(&duration))
}

#[pymodule]
fn time_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_timedelta, m)?)?;
    m.add_function(wrap_pyfunction!(timedelta_to_string, m)?)?;
    Ok(())
}
