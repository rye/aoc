fn main() {
	use std::io::BufRead;

	// Collect the input, which is a long list of positive integer weights of modules.
	let module_weights: Vec<u32> = std::io::stdin()
		.lock()
		.lines()
		.map(|line| line.unwrap().parse().unwrap())
		.collect();
}
