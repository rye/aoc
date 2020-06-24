mod one {}

mod two {}

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
}
