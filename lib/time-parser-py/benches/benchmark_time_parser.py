import timeit
from datetime import timedelta

from time_parser import parse_timedelta, timedelta_to_string

COUNT = 1_000_000

def benchmark_parse_timedelta():
    shorthand_string = "1w2d3h4m5s6ms7us8ns"
    stmt = f"parse_timedelta('{shorthand_string}')"
    setup = "from __main__ import parse_timedelta"
    total_time = timeit.timeit(stmt, setup=setup, number=COUNT)
    total_time_delta = timedelta(seconds=total_time)
    time_per_unit = timedelta(seconds=total_time / COUNT)
    print(f"time_parser:parse_timedelta('{shorthand_string}'): Total time: {timedelta_to_string(total_time_delta)}, Time per call: {timedelta_to_string(time_per_unit)}")

if __name__ == "__main__":
    benchmark_parse_timedelta()
