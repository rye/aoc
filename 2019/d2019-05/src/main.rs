fn main() {
	use std::io::BufRead;

	let program: Vec<i32> = std::io::stdin()
		.lock()
		.lines()
		.flat_map(|line: Result<String, std::io::Error>| -> Vec<i32> {
			line
				.unwrap()
				.split(',')
				.map(|n| n.parse::<i32>().unwrap())
				.collect()
		})
		.collect();

	// Part One: Compute diagnostic code
	{
		let program = program.clone();
		let mut intcode: intcode::Intcode = intcode::Intcode::from(program);
		intcode = intcode.input(1);
		intcode = intcode.run();

		let mut outputs: Vec<i32> = Vec::new();

		loop {
			if let Some(output) = intcode.output() {
				outputs.push(output);
			} else {
				break;
			}
		}

		println!("{:?}", outputs);

		println!("Part One: {}", outputs.last().expect("expected an output"));
	}

	// Part Two: Compute diagnostic code for System ID 5
	{
		let program = program.clone();
		let mut intcode: intcode::Intcode = intcode::Intcode::from(program);
		intcode = intcode.input(5);
		intcode = intcode.run();

		let mut outputs: Vec<i32> = Vec::new();

		loop {
			if let Some(output) = intcode.output() {
				outputs.push(output);
			} else {
				break;
			}
		}

		println!("{:?}", outputs);

		println!("Part Two: {}", outputs.last().expect("expected an output"));
	}
}
