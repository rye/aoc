use d2019_02::intcode_0;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn intcode_single_instruction(c: &mut Criterion) {
	c.bench_function("single instruction", |b| {
		b.iter(|| intcode_0(black_box(&[1, 0, 0, 0])))
	});
}

fn intcode_two_instructions(c: &mut Criterion) {
	c.bench_function("two instructions", |b| {
		b.iter(|| intcode_0(black_box(&[1, 0, 0, 0, 2, 0, 0, 0])))
	});
}


criterion_group!(benches, intcode_single_instruction, intcode_two_instructions);
criterion_main!(benches);
