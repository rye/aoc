use std::io::BufRead;

mod part1 {
	pub fn component_fuel_requirement(mass: &f32) -> f32 {
		(mass / 3.).floor() - 2.
	}

	#[test]
	fn component_fuel_right_12() {
		assert_eq!(component_fuel_requirement(&12.), 2.);
	}

	#[test]
	fn component_fuel_right_14() {
		assert_eq!(component_fuel_requirement(&14.), 2.);
	}

	#[test]
	fn component_fuel_right_1969() {
		assert_eq!(component_fuel_requirement(&1969.), 654.);
	}

	#[test]
	fn component_fuel_right_100756() {
		assert_eq!(component_fuel_requirement(&100756.), 33583.);
	}
}

mod part2 {
	pub fn component_fuel_requirement(mass: &f32) -> f32 {
		let mut masses: Vec<f32> = vec![super::part1::component_fuel_requirement(mass)];

		loop {
			match (masses.last().unwrap() / 3.0).floor() - 2.0 {
				n if n > 0. => masses.push(n),
				n if n <= 0. => break masses.iter().sum(),
				_ => unimplemented!(),
			}
		}
	}

	#[test]
	fn component_fuel_right_14() {
		assert_eq!(component_fuel_requirement(&14.), 2.);
	}

	#[test]
	fn component_fuel_right_1969() {
		assert_eq!(component_fuel_requirement(&1969.), 966.);
	}

	#[test]
	fn component_fuel_right_100756() {
		assert_eq!(component_fuel_requirement(&100756.), 50346.);
	}
}

fn main() {
	let values: Vec<f32> = std::io::stdin()
		.lock()
		.lines()
		.filter_map(Result::ok)
		.map(|s: String| s.parse::<f32>())
		.filter_map(Result::ok)
		.collect();

	let part1_solution: f32 = values.iter().map(part1::component_fuel_requirement).sum();
	println!("Part 1: {}", part1_solution);

	let part2_solution: f32 = values.iter().map(part2::component_fuel_requirement).sum();
	println!("Part 2: {}", part2_solution);
}
