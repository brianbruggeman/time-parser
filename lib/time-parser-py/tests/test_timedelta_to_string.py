from dataclasses import dataclass
from datetime import timedelta

import time_parser
import pytest

@dataclass
class TimeDeltaToStringTc:
    input: timedelta
    expected: str | None = None
    raises: Exception | None = None

    def _validate(self):
        actual = time_parser.timedelta_to_string(self.input)
        if self.expected is not None:
            assert actual == self.expected
        else:
            assert False, "Expected string return value"

    def validate(self):
        if self.raises:
            with pytest.raises(self.raises):
                self._validate()
        else:
            self._validate()


timedelta_to_string_happy_paths = [
    TimeDeltaToStringTc(timedelta(days=1), "1d"),
    TimeDeltaToStringTc(timedelta(weeks=1, days=2, hours=3, minutes=4, seconds=5), "1w2d3h4m5s"),
    TimeDeltaToStringTc(timedelta(days=9, seconds=11045, microseconds=6007), "1w2d3h4m5s6ms7µs"),
]

timedelta_to_string_sad_paths = [
    # Add any cases where you expect an exception to be raised
]

timedelta_to_string_test_cases = timedelta_to_string_happy_paths + timedelta_to_string_sad_paths

@pytest.mark.parametrize("tc", timedelta_to_string_test_cases)
def test_timedelta_to_string(tc: TimeDeltaToStringTc):
    tc.validate()
