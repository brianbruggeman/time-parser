from dataclasses import dataclass
from datetime import timedelta

import time_parser
import pytest

@dataclass
class ParseTimeDeltaTc:
    input: str = "1d"
    expected: timedelta | None = timedelta(days=1)
    raises: Exception | None = None

    def _validate(self):
        actual = time_parser.parse_timedelta(self.input)
        if self.expected is not None:
            assert actual == self.expected
        else:
            assert False, "Expected timedelta return value"

    def validate(self):
        if self.raises:
            with pytest.raises(self.raises):
                self._validate()
        else:
            self._validate()


parse_timedelta_happy_paths = [
    ParseTimeDeltaTc(),
    ParseTimeDeltaTc("1w2d3h4m5s", timedelta(weeks=1, days=2, hours=3, minutes=4, seconds=5)),
    ParseTimeDeltaTc("1w2d3h4m5s6ms7us", timedelta(days=9, seconds=11045, microseconds=6007)),
]

parse_timedelta_sad_paths = [
    ParseTimeDeltaTc(input="invalid", raises=ValueError),
]

parse_timedelta_test_cases = parse_timedelta_happy_paths + parse_timedelta_sad_paths

@pytest.mark.parametrize("tc", parse_timedelta_test_cases)
def test_parse_timedelta(tc: ParseTimeDeltaTc):
    tc.validate()
