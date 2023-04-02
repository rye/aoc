pub type Intermediate = Vec<i32>;
pub type Solution = usize;

enum Mode {
	PartOne,
	PartTwo,
}

pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.map(|n| n.parse())
			.collect::<Result<Vec<_>, _>>()?,
	)
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day05"),
	Some(5)
);

fn step(program: &mut Vec<i32>, offset: &mut usize, mode: Mode) {
	let jump_size = program[*offset];
	match (mode, jump_size) {
		(Mode::PartOne, _) => program[*offset] += 1,
		(Mode::PartTwo, sz) if sz >= 3 => program[*offset] -= 1,
		(Mode::PartTwo, sz) if sz < 3 => program[*offset] += 1,
		(Mode::PartTwo, _) => unreachable!(),
	}

	*offset = ((*offset as i32) + jump_size) as usize;
}

pub fn part_one(program: &Intermediate) -> Option<Solution> {
	let mut program: Vec<i32> = program.clone();
	let mut offset = 0;

	let mut counter: usize = 0_usize;

	loop {
		step(&mut program, &mut offset, Mode::PartOne);
		counter += 1;

		if offset >= program.len() {
			break;
		}
	}

	Some(counter)
}

daocutil::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("examples/day05"),
	Some(10)
);

pub fn part_two(program: &Intermediate) -> Option<Solution> {
	let mut program: Vec<i32> = program.clone();
	let mut offset = 0;

	let mut counter: usize = 0_usize;

	loop {
		step(&mut program, &mut offset, Mode::PartTwo);
		counter += 1;

		if offset >= program.len() {
			break;
		}
	}

	Some(counter)
}
