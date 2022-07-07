use leetcode::p0007_reverse_integer::Solution;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn reverse_static(c: &mut Criterion) {
    c.bench_function("reverse static", |b| b.iter(|| Solution::reverse(black_box(123454321))));
}

fn reverse_random(c: &mut Criterion) {
    let mut rand = rand::thread_rng();
    c.bench_function("reverse random", |b| b.iter(|| Solution::reverse(black_box(rand.gen()))));
}

criterion_group!{
    name = benches;
    config = Criterion::default();
    targets = reverse_static, reverse_random
}

criterion_main!(benches);