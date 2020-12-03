use std::io::{stdin, BufRead};

fn main() {
	let stdin = stdin();
	let stdin = stdin.lock();

	let _data: Vec<String> = stdin
		.lines()
		.filter_map(Result::ok)
		.map(|line| line)
		.collect();

	println!("Part One: {:?}", ());

	println!("Part Two: {:?}", ());
}

#[cfg(test)]
mod tests {}
