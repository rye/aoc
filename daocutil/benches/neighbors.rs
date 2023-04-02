use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn neighbors_benchmark(_c: &mut Criterion) {}

criterion_group!(benches, neighbors_benchmark);
criterion_main!(benches);
