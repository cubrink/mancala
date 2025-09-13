use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Callgrind, CallgrindMetrics,
    LibraryBenchmarkConfig,
};
use std::hint::black_box;

use mancala_lib::Mancala;
use perft::perft;

fn new_game() -> Mancala {
    Mancala::default()
}

#[library_benchmark]
#[bench::perft0(setup=new_game)]
fn perft0(game: Mancala) {
    black_box(perft(&game, 0));
}

#[library_benchmark]
#[bench::perft1(setup=new_game)]
fn perft1(game: Mancala) {
    black_box(perft(&game, 1));
}

#[library_benchmark]
#[bench::perft2(setup=new_game)]
fn perft2(game: Mancala) {
    black_box(perft(&game, 2));
}

#[library_benchmark]
#[bench::perft5(setup=new_game)]
fn perft5(game: Mancala) {
    black_box(perft(&game, 5));
}

library_benchmark_group!(
    name = perft_bench;
    benchmarks = perft0,perft1,perft2,perft5
);

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::default()
            .format([CallgrindMetrics::All])
        );
    library_benchmark_groups = perft_bench
);
