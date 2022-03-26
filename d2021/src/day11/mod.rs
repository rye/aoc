use std::collections::VecDeque;

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

fn neighbors<const N: usize>(y_0: u8, x_0: u8) -> impl Iterator<Item = (u8, u8)> {
	assert!(N <= u8::MAX as usize);

	let upper_bound = N as u8;

	let y_min = y_0.saturating_sub(1);
	let y_max = y_0.saturating_add(1).clamp(0, upper_bound - 1);

	let x_min = x_0.saturating_sub(1);
	let x_max = x_0.saturating_add(1).clamp(0, upper_bound - 1);

	(y_min..=y_max)
		.flat_map(move |y| (x_min..=x_max).map(move |x| (y, x)))
		.filter(move |(y, x)| y != &y_0 || x != &x_0)
}

#[cfg(test)]
mod neighbors {
	use super::neighbors;

	#[cfg(test)]
	mod _5 {

		use super::neighbors;

		#[test]
		fn corners() {
			let mut neighbors = neighbors::<5>(0, 0);

			assert_eq!(neighbors.next(), Some((0, 1)));
			assert_eq!(neighbors.next(), Some((1, 0)));
			assert_eq!(neighbors.next(), Some((1, 1)));
			assert_eq!(neighbors.next(), None);
		}

		#[test]
		fn inner_center() {
			let mut neighbors = neighbors::<5>(2, 2);

			assert_eq!(neighbors.next(), Some((1, 1)));
			assert_eq!(neighbors.next(), Some((1, 2)));
			assert_eq!(neighbors.next(), Some((1, 3)));
			assert_eq!(neighbors.next(), Some((2, 1)));
			// assert_eq!(neighbors.next(), Some((2, 2)));
			assert_eq!(neighbors.next(), Some((2, 3)));
			assert_eq!(neighbors.next(), Some((3, 1)));
			assert_eq!(neighbors.next(), Some((3, 2)));
			assert_eq!(neighbors.next(), Some((3, 3)));
			assert_eq!(neighbors.next(), None);
		}

		#[test]
		fn inner_off_center() {
			let mut neighbors = neighbors::<5>(3, 3);

			assert_eq!(neighbors.next(), Some((2, 2)));
			assert_eq!(neighbors.next(), Some((2, 3)));
			assert_eq!(neighbors.next(), Some((2, 4)));
			assert_eq!(neighbors.next(), Some((3, 2)));
			// assert_eq!(neighbors.next(), Some((3, 3)));
			assert_eq!(neighbors.next(), Some((3, 4)));
			assert_eq!(neighbors.next(), Some((4, 2)));
			assert_eq!(neighbors.next(), Some((4, 3)));
			assert_eq!(neighbors.next(), Some((4, 4)));
			assert_eq!(neighbors.next(), None);
		}

		#[test]
		fn edge_off_center() {
			let mut neighbors = neighbors::<5>(3, 4);

			assert_eq!(neighbors.next(), Some((2, 3)));
			assert_eq!(neighbors.next(), Some((2, 4)));
			assert_eq!(neighbors.next(), Some((3, 3)));
			// assert_eq!(neighbors.next(), Some((3, 4)));
			assert_eq!(neighbors.next(), Some((4, 3)));
			assert_eq!(neighbors.next(), Some((4, 4)));
			assert_eq!(neighbors.next(), None);
		}
	}
}

fn tick<const N: usize>(state: &mut State<N>) -> usize {
	assert!(N <= u8::MAX.into());

	let mut flashed: [[bool; N]; N] = [[false; N]; N];
	let mut flashes: usize = 0_usize; // This *could* be reconstructed from flashed later.

	let mut increases: VecDeque<(u8, u8)> = VecDeque::new();

	// First, the energy level of each octopus increases by 1.
	for (y, x) in (0..N).flat_map(|y| {
		(0..N).map(move |x| {
			// This "as" is safe due to assert at start.
			(y as u8, x as u8)
		})
	}) {
		increases.push_back((y, x))
	}

	// Loop until there are no more increases to handle.
	while let Some((y, x)) = increases.pop_front() {
		let y_idx: usize = y as usize;
		let x_idx: usize = x as usize;

		// If we already flashed in this spot, we're done with this increase.
		if flashed[y_idx][x_idx] {
			continue;
		} else {
			// First, bump the value.
			state.octopi[y_idx][x_idx].0 .0 += 1;

			// If that caused the octopus to have an energy level greater than 9, it flashes.
			if state.octopi[y_idx][x_idx].0 .0 > 9 {
				// Mark this cell as flashed and reset its value to 0.
				flashes += 1;
				flashed[y_idx][x_idx] = true;
				state.octopi[y_idx][x_idx].0 .0 = 0;

				// And also causes its (unflashed) neighbors to increase in level.
				for (y, x) in neighbors::<N>(y, x) {
					increases.push_back((y, x));
				}
			}
		}
	}

	flashes
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

	#[cfg(test)]
	mod _5 {
		use super::{tick, EnergyLevel, Octopus, State};

		fn state_0() -> State<5> {
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

		fn state_1() -> State<5> {
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

		fn state_2() -> State<5> {
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
		fn tick_flow() {
			let mut state = state_0();
			assert_eq!(state, state_0());
			assert_eq!(tick(&mut state), 9_usize);
			assert_eq!(state, state_1());
			assert_eq!(tick(&mut state), 0_usize);
			assert_eq!(state, state_2());
		}
	}

	#[cfg(test)]
	mod _10 {
		use super::{tick, EnergyLevel, Octopus, State};

		fn state_0() -> State<10> {
			State {
				octopi: [
					state![5 4 8 3 1 4 3 2 2 3],
					state![2 7 4 5 8 5 4 7 1 1],
					state![5 2 6 4 5 5 6 1 7 3],
					state![6 1 4 1 3 3 6 1 4 6],
					state![6 3 5 7 3 8 5 4 7 8],
					state![4 1 6 7 5 2 4 6 4 5],
					state![2 1 7 6 8 4 1 7 2 1],
					state![6 8 8 2 8 8 1 1 3 4],
					state![4 8 4 6 8 4 8 5 5 4],
					state![5 2 8 3 7 5 1 5 2 6],
				],
			}
		}

		fn state_1() -> State<10> {
			State {
				octopi: [
					state![6 5 9 4 2 5 4 3 3 4],
					state![3 8 5 6 9 6 5 8 2 2],
					state![6 3 7 5 6 6 7 2 8 4],
					state![7 2 5 2 4 4 7 2 5 7],
					state![7 4 6 8 4 9 6 5 8 9],
					state![5 2 7 8 6 3 5 7 5 6],
					state![3 2 8 7 9 5 2 8 3 2],
					state![7 9 9 3 9 9 2 2 4 5],
					state![5 9 5 7 9 5 9 6 6 5],
					state![6 3 9 4 8 6 2 6 3 7],
				],
			}
		}

		#[test]
		fn tick_flow() {
			let mut state = state_0();
			assert_eq!(state, state_0());
			assert_eq!(tick(&mut state), 0_usize);
			assert_eq!(state, state_1());
			assert_eq!(tick(&mut state), 35_usize);
		}
	}
}

type Solution = usize;

pub fn part_one(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
