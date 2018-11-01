extern crate ga_v4_flattener;
#[macro_use]
extern crate criterion;

use std::fs;
use std::path::Path;

use criterion::Criterion;
use ga_v4_flattener::to_flat_json;

fn to_flat_json_large_report_benchmark(c: &mut Criterion) {
    let data: String = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("test_reports/large_report.json"),
    )
    .unwrap();

    c.bench_function("to_flat_json_large_report", move |b| {
        b.iter(|| to_flat_json(&data))
    });
}

fn to_flat_json_multi_report_benchmark(c: &mut Criterion) {
    let data: String = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("test_reports/multiple_reports.json"),
    )
    .unwrap();

    c.bench_function("to_flat_json_multi_report", move |b| {
        b.iter(|| to_flat_json(&data))
    });
}

criterion_group!(
    benches,
    to_flat_json_large_report_benchmark,
    to_flat_json_multi_report_benchmark
);
criterion_main!(benches);
