use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use mancala_lib::Mancala;
use perft::perft;
use std::hint::black_box;

fn perft_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft");

    for depth in [0, 1, 2, 3, 5] {
        group.bench_with_input(BenchmarkId::new("depth", depth), &depth, |b, &depth| {
            b.iter_with_setup(
                || Mancala::default(),
                |game| black_box(perft(black_box(&game), black_box(depth))),
            )
        });
    }

    group.finish();
}

fn perft_benchmark_no_blackbox(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft_no_blackbox");

    for depth in [0, 1, 2, 3, 5, 8] {
        group.bench_with_input(BenchmarkId::new("depth", depth), &depth, |b, &depth| {
            b.iter_with_setup(
                || Mancala::default(), // Setup - not measured
                |game| perft(black_box(&game), black_box(depth)),
            )
        });
    }

    group.finish();
}
criterion_group!(benches, perft_benchmark, perft_benchmark_no_blackbox);
criterion_main!(benches);
