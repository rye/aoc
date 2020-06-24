mod one {
	pub fn fuel_required_for_module(mass: u32) -> u32 {
		(mass / 3) - 2
	}

	#[test]
	fn fuel_12() {
		assert_eq!(fuel_required_for_module(12), 2);
	}

	#[test]
	fn fuel_14() {
		assert_eq!(fuel_required_for_module(14), 2);
	}

	#[test]
	fn fuel_1969() {
		assert_eq!(fuel_required_for_module(1969), 654);
	}

	#[test]
	fn fuel_100756() {
		assert_eq!(fuel_required_for_module(100756), 33583);
	}
}

fn main() {
	use std::io::BufRead;

	// Collect the input, which is a long list of positive integer weights of modules.
	let module_weights: Vec<u32> = std::io::stdin()
		.lock()
		.lines()
		.map(|line| line.unwrap().parse().unwrap())
		.collect();

	// Part One: Compute the sum of the direct fuel requirements for each module.
	{
		let mut fuel_weight_sum = 0;

		for module_weight in &module_weights {
			fuel_weight_sum += one::fuel_required_for_module(*module_weight);
		}

		println!("Part One: {}", fuel_weight_sum);
	}
}
