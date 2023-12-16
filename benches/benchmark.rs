extern crate num;

use num::Complex;

use criterion::{criterion_group, criterion_main, Criterion};

use mylib::{main_multi, main_single};

fn criterion_benchmark(c: &mut Criterion) {
    let bounds = (1000, 750);
    let upper_left = Complex {
        re: -1.20,
        im: 0.35,
    };
    let lower_right = Complex { re: -1.0, im: 0.20 };

    c.bench_function("single-thread", |b| {
        b.iter(|| main_single(bounds, upper_left, lower_right))
    });
    c.bench_function("multi-thread", |b| {
        b.iter(|| main_multi(bounds, upper_left, lower_right))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
