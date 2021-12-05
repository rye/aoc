use {
	core::str::FromStr,
	std::{
		collections::{btree_map::Entry, BTreeMap, BTreeSet},
		convert::Infallible,
	},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vector(i16, i16);

impl FromStr for Vector {
	// I think you will find this FromStr quite fallible!
	// For the input, though, this works just fine.
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split(',');

		let x = split.next().map(str::parse);
		let y = split.next().map(str::parse);

		match (x, y) {
			(Some(Ok(x)), Some(Ok(y))) => Ok(Self(x, y)),
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod vector {
	use super::Vector;

	#[test]
	fn from_str_simple() {
		assert_eq!("1,2".parse(), Ok(Vector(1, 2)))
	}
}

#[derive(Clone, Copy, Debug)]
pub struct LineSegment {
	a: Vector,
	b: Vector,
}

impl LineSegment {
	fn is_horizontal(&self) -> bool {
		self.a.1 == self.b.1
	}

	fn is_vertical(&self) -> bool {
		self.a.0 == self.b.0
	}

	fn is_diagonal(&self) -> bool {
		let (dx, dy) = (self.b.0 - self.a.0, self.b.1 - self.a.1);

		dx.abs() == dy.abs()
	}

	fn points(&self) -> Box<dyn Iterator<Item = Vector>> {
		if self.is_horizontal() {
			let y = self.a.1;
			assert_eq!(y, self.b.1);

			Box::new((self.a.0..=self.b.0).map(move |x| Vector(x, y)))
		} else if self.is_vertical() {
			let x = self.a.0;
			assert_eq!(x, self.b.0);

			Box::new((self.a.1..=self.b.1).map(move |y| Vector(x, y)))
		} else if self.is_diagonal() {
			let (dir, steps): ((i16, i16), i16) = (
				(
					if self.b.0 - self.a.0 > 0 { 1 } else { -1 },
					if self.b.1 - self.a.1 > 0 { 1 } else { -1 },
				),
				(self.b.0 - self.a.0).abs(),
			);

			let x0 = self.a.0;
			let y0 = self.a.1;

			Box::new((0..=steps).map(move |i| {
				let x = x0 + (i * dir.0);
				let y = y0 + (i * dir.1);

				Vector(x, y)
			}))
		} else {
			// Technically _definitely_ reachable, but not in the problem space.
			unreachable!()
		}
	}
}

#[test]
fn points_diagonal_asc() {
	let segment = LineSegment {
		a: Vector(1, 1),
		b: Vector(3, 3),
	};
	assert!(segment.is_diagonal());

	let points: Vec<Vector> = segment.points().collect();
	assert_eq!(points, vec![Vector(1, 1), Vector(2, 2), Vector(3, 3)])
}

#[test]
fn points_diagonal_dsc() {
	let segment = LineSegment {
		a: Vector(9, 7),
		b: Vector(7, 9),
	};
	assert!(segment.is_diagonal());

	let points: Vec<Vector> = segment.points().collect();
	assert_eq!(points, vec![Vector(9, 7), Vector(8, 8), Vector(7, 9)])
}

type Intermediate = Vec<LineSegment>;

impl FromStr for LineSegment {
	type Err = Infallible;

	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let points: BTreeSet<Vector> = line
			.split(" -> ")
			.map(str::parse)
			.collect::<Result<BTreeSet<_>, Infallible>>()?;

		assert_eq!(points.len(), 2);

		let points: Vec<Vector> = points.into_iter().collect();

		Ok(LineSegment {
			a: points[0],
			b: points[1],
		})
	}
}

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.map(str::parse)
		.collect::<Result<Vec<LineSegment>, Infallible>>()
		.unwrap()
}

type Solution = usize;

pub fn part_one(segments: &Intermediate) -> Option<Solution> {
	let segments: Vec<LineSegment> = segments
		.into_iter()
		.filter(|segment| segment.is_horizontal() || segment.is_vertical())
		.copied()
		.collect();

	let mut points: BTreeMap<Vector, usize> = BTreeMap::new();

	for segment in segments {
		for point in segment.points() {
			match points.entry(point) {
				Entry::Occupied(mut e) => e.insert(e.get() + 1),
				Entry::Vacant(e) => *e.insert(1),
			};
		}
	}

	let overlaps: usize = points.iter().filter(|(&_point, &count)| count >= 2).count();

	Some(overlaps)
}

pub fn part_two(segments: &Intermediate) -> Option<Solution> {
	let mut points: BTreeMap<Vector, usize> = BTreeMap::new();

	for segment in segments {
		for point in segment.points() {
			match points.entry(point) {
				Entry::Occupied(mut e) => e.insert(e.get() + 1),
				Entry::Vacant(e) => *e.insert(1),
			};
		}
	}

	let overlaps: usize = points.iter().filter(|(&_point, &count)| count >= 2).count();

	Some(overlaps)
}
