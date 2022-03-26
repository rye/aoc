type Intermediate = State<10>;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
struct EnergyLevel(u8);

impl From<u8> for EnergyLevel {
	fn from(u8: u8) -> Self {
		Self(u8)
	}
}

impl From<char> for EnergyLevel {
	fn from(char: char) -> Self {
		EnergyLevel(match char {
			'0' => 0_u8,
			'1' => 1,
			'2' => 2,
			'3' => 3,
			'4' => 4,
			'5' => 5,
			'6' => 6,
			'7' => 7,
			'8' => 8,
			'9' => 9,
			_ => panic!("unrecognized energy level!"),
		})
	}
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
struct Octopus(EnergyLevel);

impl From<char> for Octopus {
	fn from(char: char) -> Self {
		Self(char.into())
	}
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct State<const N: usize> {
	octopi: [[Octopus; N]; N],
}

pub fn parse(input: &str) -> Intermediate {
	let octopi: [[Octopus; 10]; 10] = input
		.lines()
		.map(|line| {
			let octopi: [Octopus; 10] = line
				.chars()
				.map(|char| char.into())
				.collect::<Vec<_>>()
				.try_into()
				.expect("failed to parse 10 octopi on a line");
			octopi
		})
		.collect::<Vec<_>>()
		.try_into()
		.expect("failed to parse 10 lines of octopi");

	State { octopi }
}

fn tick<const N: usize>(state: &mut State<N>) -> usize {
	0
}

#[cfg(test)]
mod tick {
	use super::{tick, EnergyLevel, Octopus, State};

	macro_rules! state {
		($lvl:literal) => {
			Octopus($lvl.into())
		};

		[$($lvl:literal)+] => {
			[$(state!($lvl)),+]
		};
	}

	fn state_5_0() -> State<5> {
		State {
			octopi: [
				state![1 1 1 1 1],
				state![1 9 9 9 1],
				state![1 9 1 9 1],
				state![1 9 9 9 1],
				state![1 1 1 1 1],
			],
		}
	}

	fn state_5_1() -> State<5> {
		State {
			octopi: [
				state![3 4 5 4 3],
				state![4 0 0 0 4],
				state![5 0 0 0 5],
				state![4 0 0 0 4],
				state![3 4 5 4 3],
			],
		}
	}

	fn state_5_2() -> State<5> {
		State {
			octopi: [
				state![4 5 6 5 4],
				state![5 1 1 1 5],
				state![6 1 1 1 6],
				state![5 1 1 1 5],
				state![4 5 6 5 4],
			],
		}
	}

	#[test]
	fn test_0() {
		let mut state = state_5_0();

		assert_eq!(state, state_5_0());

		assert_eq!(tick(&mut state), 9_usize);

		assert_eq!(state, state_5_1());

		assert_eq!(tick(&mut state), 0_usize);

		assert_eq!(state, state_5_2());
	}
}

type Solution = usize;

pub fn part_one(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
