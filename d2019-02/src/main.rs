use std::io::BufRead;

mod part1 {
	pub fn intcode_0(pgm: &[usize]) -> usize {
		let mut memory: Vec<usize> = pgm.to_vec();
		let mut idx = 0;

		loop {
			if idx > memory.len() {
				break memory[0];
			}

			match memory[idx] {
				1 => {
					let a = memory[memory[idx + 1]];
					let b = memory[memory[idx + 2]];
					let output = memory[idx + 3];

					memory[output] = a + b;
					idx += 4;
				}
				2 => {
					let a = memory[memory[idx + 1]];
					let b = memory[memory[idx + 2]];
					let output = memory[idx + 3];

					memory[output] = a * b;
					idx += 4;
				}
				99 => break memory[0],
				_ => unimplemented!(),
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn intcode_interpreter_given_example() {
			assert_eq!(intcode_0(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]), 3500)
		}
	}
}

mod part2 {
	pub fn solve_for_value(pgm: &[usize], expected: usize) -> [usize; 2] {
		let nouns: Vec<usize> = (1..100).collect();
		let verbs: Vec<usize> = (1..100).collect();

		for noun in &nouns {
			for verb in &verbs {
				let mut pgm: Vec<usize> = pgm.to_vec();
				pgm[1] = *noun;
				pgm[2] = *verb;

				match super::part1::intcode_0(&pgm) {
					value if value == expected => return [*noun, *verb],
					_ => {}
				}
			}
		}

		unimplemented!()
	}

	#[cfg(test)]
	mod tests {}
}

fn main() {
	let starting_state: Vec<usize> = std::io::stdin()
		.lock()
		.lines()
		.filter_map(Result::ok)
		.map(|s: String| -> Vec<usize> {
			s.split(',')
				.map(|k: &str| k.parse::<usize>())
				.map(Result::unwrap)
				.collect()
		})
		.flatten()
		.collect();

	let mut program: Vec<usize> = starting_state;
	program[1] = 12;
	program[2] = 2;

	println!("Part 1: {}", part1::intcode_0(&program));

	let [noun, verb]: [usize; 2] = part2::solve_for_value(&program, 19690720);

	println!("Part 2: {:?}", 100 * noun + verb);
}
