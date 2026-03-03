//! Benchmarks for AgenticReality.

use criterion::{criterion_group, criterion_main, Criterion};

fn engine_benchmarks(c: &mut Criterion) {
    c.bench_function("create_engine", |b| {
        b.iter(|| {
            let _engine = agentic_reality::engine::RealityEngine::new();
        })
    });
}

criterion_group!(benches, engine_benchmarks);
criterion_main!(benches);
