use std::io::BufRead;

mod part1 {
	use std::collections::HashSet;
	use core::convert::TryInto;

	#[derive(Debug, PartialEq)]
	pub enum Direction {
		Up,
		Down,
		Left,
		Right,
	}

	impl From<Direction> for Vec2d {
		fn from(d: Direction) -> Vec2d {
			match d {
				Direction::Up => Vec2d(0, 1),
				Direction::Down => Vec2d(0, -1),
				Direction::Left => Vec2d(-1, 0),
				Direction::Right => Vec2d(1, 0),
			}
		}
	}

	impl core::ops::Mul<Vec2d> for i32 {
		type Output = Vec2d;

		fn mul(self, vec: Vec2d) -> Self::Output {
			Vec2d(self * vec.0, self * vec.1)
		}
	}

	impl core::ops::Add<Vec2d> for Vec2d {
		type Output = Vec2d;

		fn add(self, vec: Vec2d) -> Self::Output {
			Vec2d(self.0 + vec.0, self.1 + vec.1)
		}
	}

	impl<'a> core::ops::Add<Vec2d> for &'a Vec2d {
		type Output = Vec2d;

		fn add(self, vec: Vec2d) -> Self::Output {
			Vec2d(self.0 + vec.0, self.1 + vec.1)
		}
	}

	#[derive(Debug)]
	pub enum DirectionParseError {
		InvalidCharacter,
	}

	impl core::convert::TryFrom<char> for Direction {
		type Error = DirectionParseError;

		fn try_from(c: char) -> Result<Self, Self::Error> {
			match c {
				'U' => Ok(Direction::Up),
				'D' => Ok(Direction::Down),
				'L' => Ok(Direction::Left),
				'R' => Ok(Direction::Right),
				_ => Err(DirectionParseError::InvalidCharacter),
			}
		}
	}

	#[derive(Debug)]
	pub enum SegmentParseError {
		MissingComponent,
		Malformed,
	}

	impl core::convert::From<DirectionParseError> for SegmentParseError {
		fn from(e: DirectionParseError) -> Self {
			match e {
				DirectionParseError::InvalidCharacter => Self::Malformed,
			}
		}
	}

	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
	pub struct Vec2d(pub i32, pub i32);

	impl Vec2d {
		pub fn manhattan_distance(&self, other: &Self) -> i32 {
			(self.0 - other.0).abs() + (self.1 - other.1).abs()
		}
	}

	#[derive(Debug)]
	pub struct Segment {
		direction: Direction,
		distance: i32,
	}

	impl Segment {
		#[allow(dead_code)]
		pub fn direction(&self) -> &Direction {
			&self.direction
		}

		#[allow(dead_code)]
		pub fn distance(&self) -> &i32 {
			&self.distance
		}
	}

	#[derive(Clone, Debug)]
	pub struct Wire {
		points: HashSet<Vec2d>,
	}

	impl Wire {
		pub fn intersection(&self, other: &Wire) -> Vec<Vec2d> {
			self.points.intersection(&other.points).cloned().collect()
		}

		pub fn signal_distance_to(&self, other: &Wire) -> i32 {
			0
		}
	}

	impl From<Vec<Segment>> for Wire {
		fn from(segments: Vec<Segment>) -> Wire {
			let origin: Vec2d = Vec2d(0, 0);
			let mut points: HashSet<Vec2d> = HashSet::new();
			points.insert(origin);

			let mut last: Vec2d = origin;

			for segment in segments {
				let direction: Direction = segment.direction;
				let direction: Vec2d = direction.into();


				for i in 1..=segment.distance {
					points.insert(last + i * direction);
				}

				last = last + segment.distance * direction;
			}

			Wire { points }
		}
	}

	impl core::str::FromStr for Segment {
		type Err = SegmentParseError;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			let direction: char = s.chars().nth(0).unwrap();
			let distance: i32 = s[1..].parse().unwrap();

			let direction: Direction = direction
				.try_into()
				.map_err(|e: DirectionParseError| -> SegmentParseError { e.into() })?;

			Ok(Segment {
				direction,
				distance,
			})
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn can_parse_segment() {
			let segment: Segment = "D42".parse().unwrap();
			assert_eq!(segment.direction(), &Direction::Down);
			assert_eq!(segment.distance(), &42_i32);
		}

		#[test]
		fn can_add_vecs() {
			let v1: Vec2d = Vec2d(0, 0);
			let v2: Vec2d = Vec2d(3, 5);

			assert_eq!(v1 + v2, Vec2d(3, 5))
		}
	}
}

mod part2 {
	#[cfg(test)]
	mod tests {}
}

fn main() {
	use part1::Vec2d;

	let wires: Vec<part1::Wire> = std::io::stdin()
		.lock()
		.lines()
		.filter_map(Result::ok)
		.map(|s: String| -> Vec<part1::Segment> {
			s.split(',')
				.map(|k: &str| k.parse::<part1::Segment>())
				.map(Result::unwrap)
				.collect()
		})
		.map(Into::into)
		.collect();

	let a = &wires[0];
	let b = &wires[1];
	let intersections: Vec<part1::Vec2d> = a.intersection(b);

	let origin: Vec2d = Vec2d(0, 0);

	let mut intersection_distances: Vec<(Vec2d, i32)> = intersections.iter().map(|intersection: &Vec2d| -> (Vec2d, i32) {
		(intersection.clone(), intersection.manhattan_distance(&origin))
	}).collect();

	intersection_distances.sort_by(|a: &(Vec2d, i32), b: &(Vec2d, i32)| -> core::cmp::Ordering {
		a.1.cmp(&b.1)
	});

	println!("Part 1: {:?}", intersection_distances[1].1);

	let mut intersection_signal_distances: Vec<(Vec2d, i32, i32, i32)> = intersections.iter().map(|intersection: &Vec2d| {
		let a: i32 = a.signal_distance_to(intersection);
		let b: i32 = b.signal_distance_to(intersection);
		(intersection.clone(), a, b, a + b)
	});

	// println!("Part 2:");
}
