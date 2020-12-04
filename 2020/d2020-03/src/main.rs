use std::io::{stdin, BufRead};

fn main() {
	let stdin = stdin();
	let stdin = stdin.lock();

	let data: Vec<Vec<char>> = stdin
		.lines()
		.filter_map(Result::ok)
		.map(|line| line.chars().collect())
		.collect();

	{
		let mut x = 0;
		let mut y = 0;
		let mut trees = 0;

		let data = data.clone();

		loop {
			let c: char = data[y][x];

			if c == '#' {
				trees += 1;
			}

			if y >= data.len() - 1 {
				break;
			} else {
				x += 3;
				y += 1;
				if x >= data[y].len() {
					x %= data[y].len();
				}
			}
		}

		println!("Part One: {:?}", trees);
	}

	{
		let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
		let data = data.clone();
		let mut result: usize = 1;

		for slope in slopes {
			let mut x = 0;
			let mut y = 0;
			let mut tree_hits = 0;

			loop {
				let c: char = data[y][x];

				if c == '#' {
					tree_hits += 1;
				}

				if y >= data.len() - 1 {
					break;
				} else {
					x += slope.0;
					y += slope.1;
					if x >= data[y].len() {
						x %= data[y].len();
					}
				}
			}

			result *= tree_hits;
		}

		println!("Part Two: {:?}", result);
	}
}

#[cfg(test)]
mod tests {}
