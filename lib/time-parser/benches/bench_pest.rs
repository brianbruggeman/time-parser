use std::time::Duration as StdDuration;

use chrono::Duration as ChronoDuration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use time_parser::{parse_duration, parse_duration_hms, parse_duration_shorthand, DurationFormatter};

fn configure_criterion() -> Criterion {
    Criterion::default().noise_threshold(0.05) // Set noise threshold to 5%
}

fn benchmark_parse_duration_with_pest(c: &mut Criterion) {
    let test_string_hms = "02:03:04";
    let test_string_shorthand = "1w2d3h4m5s6ms7us8ns";

    c.bench_function("parse_duration_with_pest_hms", |b| b.iter(|| parse_duration_hms(black_box(test_string_hms))));

    c.bench_function("parse_duration_with_pest_shorthand", |b| b.iter(|| parse_duration_shorthand(black_box(test_string_shorthand))));

    c.bench_function("parse_duration_with_pest", |b| b.iter(|| parse_duration(black_box(test_string_shorthand))));

    c.bench_function("duration_formatter_std", |b| b.iter(|| StdDuration::parse(black_box(test_string_shorthand))));

    c.bench_function("duration_formatter_chrono", |b| b.iter(|| ChronoDuration::parse(black_box(test_string_shorthand))));
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = benchmark_parse_duration_with_pest
);

criterion_main!(benches);
