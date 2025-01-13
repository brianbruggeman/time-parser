[![Build Status](https://travis-ci.org/brianbruggeman/time-parser.svg?branch=master)](https://travis-ci.org/brianbruggeman/time-parser)

# time-parser

A simple library for parsing strings into various time formats.

## Features

- Parse durations in various formats
    - Shorthand (e.g. 1d2h3m4s)
    - Hours, minutes, seconds (e.g. 02:03:04)

- Shorthand supports subsecond precision
    - up to nanoseconds in Rust
    - up to microseconds in Python
    - when converting to Python from Rust, the nanoseconds will be truncated


### Acceptible unit strings

| Unit | Options |
| ---- | ------- |
| week | w, week, weeks |
| day | d, day, days |
| hour | h, hr, hour, hours |
| minute | m, min, minute, minutes |
| second | s, sec, second, seconds |
| millisecond | ms, millisecond, milliseconds |
| microsecond | u, µ, us, µs, microsecond, microseconds |
| nanosecond | n, ns, nanosecond, nanoseconds |

When parsing, the unit string is case sensitive.  These are all valid for `1 day`:
- `1d`
- `1 d`
- `1 day`
- `1day`
- `1 days`
- `1days`


## Usage

Parse a duration string into a `std::time::Duration` and `chrono::Duration`.  The function call will return `std::time::Duration` when the string is valid.  For `chrono` support, use the `DurationFormatter`.

```rust
use time_parser::{parse_duration, duration_to_string, DurationFormatter};

let duration_string = "1d2h3m4s";

// std::time::Duration
let duration: std::time::Duration = parse_duration(duration_string).expect("duration must be valid");
let std_duration = duration_string.parse::<std::time::Duration>().expect("duration must be valid");

assert_eq!(duration, std_duration);


// chrono::Duration
let chrono_duration = duration_string.parse::<chrono::Duration>().expect("duration must be valid");
```

The python support only contains the `parse_timedelta` function and is limited to the timedelta type.

```python
from datetime import timedelta
from time_parser import parse_timedelta

timedelta_string = "1d2h3m4s"
duration: timedelta = parse_timedelta(timedelta_string)
```

#### Installation

For Rust, add this to your `Cargo.toml`:

```toml
[dependencies]
time-parser = { git = "https://github.com/brianbruggeman/time-parser.git", rev = "0.1.0" }
```

For Python, add this to your `pyproject.toml` under the `[project.dependencies]` section:

```toml
[project.dependencies]
time-parser = { url = "git+https://github.com/brianbruggeman/time-parser.git@0.1.0" }
```

## License

Licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)