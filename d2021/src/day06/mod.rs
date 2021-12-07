use core::{
	convert::Infallible,
	ops::{Index, IndexMut},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimerValue(u8);

#[derive(Clone, Copy)]
pub struct School([usize; 9]);

impl Index<TimerValue> for School {
	type Output = usize;

	fn index(&self, index: TimerValue) -> &Self::Output {
		&self.0[index.0 as usize]
	}
}

impl IndexMut<TimerValue> for School {
	fn index_mut(&mut self, index: TimerValue) -> &mut Self::Output {
		&mut self.0[index.0 as usize]
	}
}

impl core::str::FromStr for TimerValue {
	type Err = Infallible;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let timer_value: u8 = value.parse().expect("failed to parse fish value");
		Ok(Self(timer_value))
	}
}

type Intermediate = School;

pub fn parse(input: &str) -> Intermediate {
	let fish_values: Vec<TimerValue> = input
		.trim()
		.split(',')
		.map(str::parse)
		.collect::<Result<Vec<_>, _>>()
		.expect("failed to parse input");

	let mut school = School([0; 9]);

	for fish_value in fish_values {
		school[fish_value] += 1;
	}

	school
}

type Solution = usize;

fn update_school(school: &mut School) {
	let new_values: [usize; 9] = [
		school[TimerValue(1)],
		school[TimerValue(2)],
		school[TimerValue(3)],
		school[TimerValue(4)],
		school[TimerValue(5)],
		school[TimerValue(6)],
		school[TimerValue(7)] + school[TimerValue(0)],
		school[TimerValue(8)],
		school[TimerValue(0)],
	];

	school.0 = new_values;
}

fn school_size(school: &School) -> usize {
	school.0.iter().sum()
}

fn simulate(school: &mut School, cycles: usize) {
	for _ in 0..cycles {
		update_school(school);
	}
}

pub fn part_one(school: &Intermediate) -> Option<Solution> {
	let mut school: School = *school;

	simulate(&mut school, 80);

	Some(school_size(&school))
}

pub fn part_two(school: &Intermediate) -> Option<Solution> {
	let mut school: School = *school;

	simulate(&mut school, 256);

	Some(school_size(&school))
}
