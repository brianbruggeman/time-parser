import timeit
from datetime import timedelta
from parse_timedelta import (
    parse_timedelta,
    parse_timedelta_hms,
    parse_timedelta_shorthand
)
from time_parser import timedelta_to_string

COUNT = 1_000_000
def format_timedelta_with_ns(seconds: float) -> str:
    nanoseconds = int(seconds * 1e9) % 1_000  # Get the nanos by first going to full nanoseconds then modulating
    td = timedelta(seconds=seconds - nanoseconds / 1e9)
    string = timedelta_to_string(td)
    return f"{string}{nanoseconds}ns" if nanoseconds > 0 else string

def display(prefix: str, total_time: float):
    formatted_total_time = format_timedelta_with_ns(total_time)
    formatted_time_per_call = format_timedelta_with_ns(total_time / COUNT)
    print(f"{prefix}: Total time: {formatted_total_time}, Time per call: {formatted_time_per_call}")

def benchmark_parse_timedelta_hms():
    hms_string = "12:34:56"
    stmt = f"parse_timedelta_hms('{hms_string}')"
    setup = "from __main__ import parse_timedelta_hms"
    total_time = timeit.timeit(stmt, setup=setup, number=COUNT)
    display(f"parse_duration_regex_hms_{hms_string}", total_time)

def benchmark_parse_timedelta_shorthand():
    shorthand_string = "1w2d3h4m5s"
    stmt = f"parse_timedelta_shorthand('{shorthand_string}')"
    setup = "from __main__ import parse_timedelta_shorthand"
    total_time = timeit.timeit(stmt, setup=setup, number=COUNT)
    display(f"parse_duration_regex_shorthand_{shorthand_string}", total_time)

def benchmark_parse_timedelta():
    shorthand_string = "1w2d3h4m5s6ms7us8ns"
    stmt = f"parse_timedelta('{shorthand_string}')"
    setup = "from __main__ import parse_timedelta"
    total_time = timeit.timeit(stmt, setup=setup, number=COUNT)
    display(f"parse_duration_regex_{shorthand_string}", total_time)

if __name__ == "__main__":
    benchmark_parse_timedelta_hms()
    benchmark_parse_timedelta_shorthand()
    benchmark_parse_timedelta()