use std::convert::Infallible;

#[derive(Clone)]
pub struct Fish {
	timer_value: u8,
}

#[derive(Debug)]
enum FishTickResult {
	CreateNewFish,
}

impl Fish {
	fn tick(&mut self) -> Option<FishTickResult> {
		if self.timer_value == 0 {
			self.timer_value = 6;
			Some(FishTickResult::CreateNewFish)
		} else {
			self.timer_value -= 1;
			None
		}
	}
}

impl core::str::FromStr for Fish {
	type Err = Infallible;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let timer_value: u8 = value.parse().expect("failed to parse fish value");
		Ok(Self { timer_value })
	}
}

type Intermediate = Vec<Fish>;

pub fn parse(input: &str) -> Intermediate {
	input
		.trim()
		.split(',')
		.map(str::parse)
		.collect::<Result<Vec<_>, _>>()
		.expect("failed to parse input")
}

type Solution = usize;

fn simulate(school: &mut Vec<Fish>, cycles: usize) {
	for cycle in 0..cycles {
		let results: Vec<FishTickResult> = school.iter_mut().filter_map(|fish| fish.tick()).collect();

		for result in &results {
			match result {
				FishTickResult::CreateNewFish => school.push(Fish { timer_value: 8_u8 }),
			}
		}

		println!("{}, {}", cycle, school.len());
	}
}

pub fn part_one(school: &Intermediate) -> Option<Solution> {
	let mut school: Vec<Fish> = school.to_vec();

	simulate(&mut school, 80);

	Some(school.len())
}

pub fn part_two(school: &Intermediate) -> Option<Solution> {
	let mut school: Vec<Fish> = school.to_vec();

	simulate(&mut school, 256);

	Some(school.len())
}
