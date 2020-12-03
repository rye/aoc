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

	// Part One: Compute value left at position 0.
	{
		let program = {
			let mut program = program.clone();
			program[1] = 12;
			program[2] = 2;
			program
		};

		let mut intcode: intcode::Intcode = intcode::Intcode::from(program);
		let final_memory = intcode.run().data();
		println!("Part One: {}", final_memory[0]);
	}

	// Part Two: Figure out what pair of inputs produces the output 19690720.
	{
		let mut done = false;
		let mut pair = (0, 0);

		for noun in 0..=100 {
			for verb in 0..=100 {
				let program = {
					let mut program = program.clone();
					program[1] = noun;
					program[2] = verb;
					program
				};

				let output = intcode::Intcode::from(program).run().data()[0];

				if output == 19690720 {
					done = true;
					pair = (noun, verb);
					break;
				}
			}
			if done {
				break;
			}
		}

		println!("Part Two: {}", pair.0 * 100 + pair.1);
	}
}
