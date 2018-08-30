extern crate ga_v4_flattener;
#[macro_use]
extern crate criterion;

use std::fs;
use std::path::PathBuf;

use criterion::Criterion;
use ga_v4_flattener::to_delimited;

fn to_delimited_large_report_benchmark(c: &mut Criterion) {
    let data: String = fs::read_to_string(PathBuf::from(format!(
        "{}{}",
        env!("CARGO_MANIFEST_DIR"),
        "/test_reports/large_report.json"
    ))).unwrap();

    c.bench_function("to_delimited_large_report", move |b| {
        b.iter(|| to_delimited(&data, ","))
    });
}

fn to_delimited_multi_report_benchmark(c: &mut Criterion) {
    let data: String = fs::read_to_string(PathBuf::from(format!(
        "{}{}",
        env!("CARGO_MANIFEST_DIR"),
        "/test_reports/multiple_reports.json"
    ))).unwrap();

    c.bench_function("to_delimited_multi_report", move |b| {
        b.iter(|| to_delimited(&data, ","))
    });
}

criterion_group!(
    benches,
    to_delimited_large_report_benchmark,
    to_delimited_multi_report_benchmark
);
criterion_main!(benches);
