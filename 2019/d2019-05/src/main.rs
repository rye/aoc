use std::io::BufRead;

mod part1 {
	pub use d2019_05::intcode_0;

	#[cfg(test)]
	mod tests {
		use super::*;
	}
}

mod part2 {
	#[cfg(test)]
	mod tests {}
}

fn main() {
	let filename: &String = &std::env::args().collect::<Vec<String>>()[1];
	let program: String = std::fs::read_to_string(filename).unwrap().trim().to_string();
	let program: Vec<i32> = program.split(',')
		.map(|k: &str| k.parse::<i32>())
		.map(Result::unwrap)
		.collect();

	println!("Part 1: {:?}", part1::intcode_0(&program));

//	let [noun, verb]: [usize; 2] = part2::solve_for_value(&program, 19690720);

//	println!("Part 2: {:?}", 100 * noun + verb);
}
