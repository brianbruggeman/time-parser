import re
from datetime import timedelta
from typing import Optional

def parse_timedelta_hms(interval: str) -> None | timedelta:
    pattern = r"^(\d+):(\d+):(\d+)$"
    match = re.match(pattern, interval)

    if match:
        hours = int(match.group(1))
        minutes = int(match.group(2))
        seconds = int(match.group(3))
        return timedelta(hours=hours, minutes=minutes, seconds=seconds)
    else:
        return None

def parse_timedelta_shorthand(interval: str) -> None | timedelta:
    pattern = r"(\d+(?:_\d{3})*)([wdhms])"
    matches = re.finditer(pattern, interval.lower())
    delta = timedelta()
    has_valid_unit = False

    for match in matches:
        has_valid_unit = True
        base_number_str = match.group(1).replace('_', '')
        base_number = int(base_number_str)

        unit = match.group(2)

        if unit == 'w':
            delta += timedelta(weeks=base_number)
        elif unit == 'd':
            delta += timedelta(days=base_number)
        elif unit == 'h':
            delta += timedelta(hours=base_number)
        elif unit == 'm':
            delta += timedelta(minutes=base_number)
        elif unit == 's':
            delta += timedelta(seconds=base_number)

    if not has_valid_unit:
        return None

    return delta

def parse_timedelta(interval: str) -> None | timedelta:
    result = parse_timedelta_hms(interval) or parse_timedelta_shorthand(interval)
    return result
