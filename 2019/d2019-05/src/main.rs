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
		println!("Part One: {:?}", ());
	}

	// Part Two: Compute diagnostic code for System ID 5
	{
		println!("Part Two: {:?}", ());
	}
}
