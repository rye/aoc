use itertools::Itertools;

use crate::intcode::Intcode;

pub type Intermediate = Vec<i32>;
pub type Output = i32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let program: Vec<i32> = input
		.lines()
		.flat_map(|line| -> Vec<i32> { line.split(',').map(|n| n.parse::<i32>().unwrap()).collect() })
		.collect();

	Ok(program)
}

#[must_use]
pub fn part_one(program: &Intermediate) -> Option<Output> {
	let permutations = (0..5).permutations(5);

	let mut max: Option<i32> = None;

	for permutation in permutations {
		// Setting for A is 0.

		// Program takes inputs in this order: phase setting, value.

		let [a, b, c, d, e] = [
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
			Intcode::from(program.clone()),
		];

		let a = a.input(permutation[0]).input(0);
		let b = b.input(permutation[1]);
		let c = c.input(permutation[2]);
		let d = d.input(permutation[3]);
		let e = e.input(permutation[4]);

		// Run A, feed into B
		let mut a = a.run();
		let b = b.input(a.output().expect("expected output from A"));

		// Run B, feed into C
		let mut b = b.run();
		let c = c.input(b.output().expect("expected output from B"));

		// Run C, feed into D
		let mut c = c.run();
		let d = d.input(c.output().expect("expected output from C"));

		// Run D, feed into E
		let mut d = d.run();
		let e = e.input(d.output().expect("expected output from D"));

		let mut e = e.run();
		let output = e.output().expect("expected output from E");

		println!("Setting {:?} produces output {output}", permutation);

		if let Some(max_so_far) = max {
			if output > max_so_far {
				max.replace(output);
			}
		} else {
			max.replace(output);
		}
	}

	max
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
