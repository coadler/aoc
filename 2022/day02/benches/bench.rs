use std::str;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day02::part1;
use day02::part2;

fn criterion_benchmark(c: &mut Criterion) {
    let input = unsafe { str::from_utf8_unchecked(include_bytes!("../input.txt")) };
    c.bench_function("part1", |b| {
        b.iter(|| part1::run(black_box(input)));
    });
    c.bench_function("part2", |b| {
        b.iter(|| part2::run(black_box(input)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
