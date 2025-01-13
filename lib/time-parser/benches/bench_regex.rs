use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod durations_regex;
use durations_regex::{parse_duration, parse_duration_hms, parse_duration_shorthand};

fn bench_parse_duration(c: &mut Criterion) {
    let hms_string = "12:34:56";
    let id = format!("parse_duration_regex_hms_{hms_string}");
    c.bench_function(&id, |b| {
        let test_string = hms_string;
        b.iter(|| {
            parse_duration_hms(black_box(test_string)).expect("interval must be valid for this test");
        })
    });

    let shorthand_string = "1w2d3h4m5s";
    let id = format!("parse_duration_regex_shorthand_{shorthand_string}");
    c.bench_function(&id, |b| {
        let test_string = shorthand_string;
        b.iter(|| {
            parse_duration_shorthand(black_box(test_string)).expect("interval must be valid for this test");
        })
    });

    let shorthand_string = "1w2d3h4m5s6ms7us8ns";
    let id = format!("parse_duration_regex_{shorthand_string}");
    c.bench_function(&id, |b| {
        let test_string = shorthand_string;
        b.iter(|| {
            parse_duration(black_box(test_string)).expect("interval must be valid for this test");
        })
    });
}

criterion_group!(benches, bench_parse_duration);
criterion_main!(benches);
