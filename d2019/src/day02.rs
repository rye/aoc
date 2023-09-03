use crate::intcode;

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
	let program = {
		let mut program = program.clone();
		program[1] = 12;
		program[2] = 2;
		program
	};

	let mut intcode: intcode::Intcode = intcode::Intcode::from(program);
	intcode = intcode.run();
	let final_memory = intcode.data();

	Some(final_memory[0])
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
