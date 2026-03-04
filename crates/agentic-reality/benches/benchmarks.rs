//! Benchmarks for AgenticReality.

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_create_engine(c: &mut Criterion) {
    c.bench_function("create_engine", |b| {
        b.iter(|| {
            let _engine = agentic_reality::engine::RealityEngine::new();
        })
    });
}

fn bench_context_summary(c: &mut Criterion) {
    c.bench_function("context_summary", |b| {
        let engine = agentic_reality::engine::RealityEngine::new();
        b.iter(|| {
            let _summary = engine.reader().get_context_summary();
        })
    });
}

fn bench_is_coherent(c: &mut Criterion) {
    c.bench_function("is_coherent", |b| {
        let engine = agentic_reality::engine::RealityEngine::new();
        b.iter(|| {
            let _coherent = engine.reader().is_coherent();
        })
    });
}

fn bench_get_unified_timeline(c: &mut Criterion) {
    c.bench_function("get_unified_timeline", |b| {
        let engine = agentic_reality::engine::RealityEngine::new();
        b.iter(|| {
            let _timeline = engine.reader().get_unified_timeline();
        })
    });
}

fn bench_set_stakes_level(c: &mut Criterion) {
    c.bench_function("set_stakes_level_minimal", |b| {
        b.iter(|| {
            let mut engine = agentic_reality::engine::RealityEngine::new();
            let _result = engine.writer().set_stakes_level(
                agentic_reality::types::stakes::StakesLevel::Minimal {
                    can_experiment: true,
                },
            );
        })
    });
}

criterion_group!(
    benches,
    bench_create_engine,
    bench_context_summary,
    bench_is_coherent,
    bench_get_unified_timeline,
    bench_set_stakes_level
);
criterion_main!(benches);
