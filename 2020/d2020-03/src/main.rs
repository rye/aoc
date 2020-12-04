use std::io::{stdin, BufRead};

fn slope(data: &Vec<Vec<char>>, (dx, dy): (usize, usize)) -> usize {
	let mut position = (0, 0);
	let mut hits = 0;

	loop {
		let c: char = data[position.1][position.0];

		if c == '#' {
			hits += 1;
		}

		if position.1 < data.len() - 1 {
			position.0 += dx;
			position.1 += dy;

			if position.0 >= data[position.1].len() {
				position.0 %= data[position.1].len();
			}
		} else {
			break;
		}
	}

	hits
}

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

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(test)]
	mod slope {
		use super::slope;

		#[cfg(test)]
		fn test_slope() -> Vec<Vec<char>> {
			let data = include_str!("test-data");
			let data: Vec<Vec<char>> = data
				.split("\n")
				.filter(|s| s.len() > 0)
				.map(|line| line.chars().collect())
				.collect();

			data
		}

		#[test]
		fn slope_3_1_correct() {
			assert_eq!(slope(&test_slope(), (3, 1)), 7);
		}

		#[test]
		fn slope_1_1_correct() {
			assert_eq!(slope(&test_slope(), (1, 1)), 2);
		}

		#[test]
		fn slope_5_1_correct() {
			assert_eq!(slope(&test_slope(), (5, 1)), 3);
		}

		#[test]
		fn slope_7_1_correct() {
			assert_eq!(slope(&test_slope(), (7, 1)), 4);
		}

		#[test]
		fn slope_1_2_correct() {
			assert_eq!(slope(&test_slope(), (1, 2)), 2);
		}
	}
}
