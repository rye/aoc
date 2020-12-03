use std::io::{stdin, BufRead};

fn main() {
	let stdin = stdin();
	let stdin = stdin.lock();

	let data: Vec<String> = stdin
		.lines()
		.filter_map(Result::ok)
		.map(|line| line)
		.collect();

	let width = data[0].len();
	let height = data.len();

	{
		let mut x = 0;
		let mut y = 0;
		let mut trees = 0;

		loop {
			let chars: Vec<char> = data[y].chars().collect();
			let c: char = chars[x];

			if c == '#' {
				trees += 1;
			}

			if y == data.len() - 1 {
				break;
			} else {
				x += 3;
				y += 1;
				if x >= chars.len() {
					x %= chars.len();
				}
			}
		}

		println!("Part One: {:?}", trees);
	}

	println!("Part Two: {:?}", ());
}

#[cfg(test)]
mod tests {}
