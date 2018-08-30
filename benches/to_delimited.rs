extern crate ga_v4_flattener;
#[macro_use]
extern crate criterion;

use std::fs;
use std::path::PathBuf;

use criterion::Criterion;
use ga_v4_flattener::to_delimited;

fn criterion_benchmark(c: &mut Criterion) {
    let data: String = fs::read_to_string(PathBuf::from(format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/test_reports/large_report.json"
        ))).unwrap();

    c.bench_function("to_delimited", move |b| b.iter(|| to_delimited(&data, ",")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
