use std::io::{stdin, BufRead};

use d2020::day03::slope;

fn main() {
	let stdin = stdin();
	let stdin = stdin.lock();

	let data: Vec<Vec<char>> = stdin
		.lines()
		.filter_map(Result::ok)
		.map(|line| line.chars().collect())
		.collect();

	{
		println!("Part One: {:?}", slope(&data, (3, 1)));
	}

	{
		let result = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
			.drain(..)
			.map(|trajectory| slope(&data, trajectory))
			.fold(1, |acc, x| acc * x);

		println!("Part Two: {:?}", result);
	}
}
