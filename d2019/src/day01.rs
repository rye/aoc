pub type Intermediate = Vec<u32>;
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let module_weights: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

	Ok(module_weights)
}

#[must_use]
pub fn part_one(module_weights: &Intermediate) -> Option<Output> {
	pub fn fuel_required_for_module(mass: u32) -> u32 {
		(mass / 3) - 2
	}

	let mut fuel_weight_sum = 0;

	for module_weight in module_weights {
		fuel_weight_sum += fuel_required_for_module(*module_weight);
	}

	Some(fuel_weight_sum)
}

daocutil::generate_example_tests!(
	parse, part_one,
	fuel_12 | "12" => Some(2),
	fuel_14 | "14" => Some(2),
	fuel_1969 | "1969" => Some(654),
	fuel_100756 | "100756" => Some(33583),
);

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
