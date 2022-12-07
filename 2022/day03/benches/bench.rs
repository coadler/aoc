use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day03::part1;
use day03::part2;

fn criterion_benchmark(c: &mut Criterion) {
    let bytes = include_bytes!("../input.txt");
    c.bench_function("part1", |b| {
        b.iter(|| part1::run(black_box(bytes)));
    });
    c.bench_function("part2", |b| {
        b.iter(|| part2::run(black_box(bytes)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
