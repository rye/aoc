mod two {
	pub fn fuel_required_for_module(mass: u32) -> u32 {
		// Compute the required fuel (if any) for a given mass.
		let fuel_for_mass = |mass: u32| (mass / 3).checked_sub(2);

		// A running total.
		let mut total: u32 = 0;

		// Copy and mutify the mass variable.
		let mut mass = mass;

		// This loop runs until no more fuel is required for the remaining unaccounted mass.
		loop {
			// If we need some more fuel for the mass,
			if let Some(part) = fuel_for_mass(mass) {
				// add the required fuel to the total.
				total += part;
				// Now we only have to compute for the mass we added.
				mass = part;
			} else {
				// If we didn't need any additional fuel, we're done.
				break;
			}
		}

		total
	}

	#[test]
	fn fuel_14() {
		assert_eq!(fuel_required_for_module(14), 2);
	}

	#[test]
	fn fuel_1969() {
		assert_eq!(fuel_required_for_module(1969), 966);
	}

	#[test]
	fn fuel_100756() {
		assert_eq!(fuel_required_for_module(100756), 50346);
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

	// Part Two: Compute the sum of the *total* fuel requirements for each module.
	{
		let mut fuel_weight_sum = 0;

		for module_weight in &module_weights {
			fuel_weight_sum += two::fuel_required_for_module(*module_weight);
		}

		println!("Part Two: {}", fuel_weight_sum);
	}
}
