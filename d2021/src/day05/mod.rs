use std::{
	collections::{BTreeMap, BTreeSet},
	convert::Infallible,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
	x: u16,
	y: u16,
}

impl core::str::FromStr for Point {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split(',');

		let x = split.next().map(str::parse);
		let y = split.next().map(str::parse);

		match (x, y) {
			(Some(Ok(x)), Some(Ok(y))) => Ok(Self { x, y }),
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod point {
	use super::Point;

	#[test]
	fn from_str_simple() {
		assert_eq!("1,2".parse(), Ok(Point { x: 1, y: 2 }))
	}
}

#[derive(Clone, Copy, Debug)]
pub struct LineSegment {
	a: Point,
	b: Point,
}

impl LineSegment {
	fn is_horizontal(&self) -> bool {
		self.a.y == self.b.y
	}

	fn is_vertical(&self) -> bool {
		self.a.x == self.b.x
	}

	fn points(&self) -> Box<dyn Iterator<Item = Point>> {
		if self.is_horizontal() {
			let y = self.a.y;
			assert_eq!(y, self.b.y);

			Box::new((self.a.x..=self.b.x).map(move |x| Point { x, y }))
		} else if self.is_vertical() {
			let x = self.a.x;
			assert_eq!(x, self.b.x);

			Box::new((self.a.y..=self.b.y).map(move |y| Point { x, y }))
		} else {
			unreachable!()
		}
	}
}

type Intermediate = Vec<LineSegment>;

fn parse_line(line: &str) -> Result<LineSegment, Infallible> {
	let points: BTreeSet<Point> = line
		.split(" -> ")
		.map(str::parse)
		.collect::<Result<BTreeSet<_>, Infallible>>()?;

	assert_eq!(points.len(), 2);

	let points: Vec<Point> = points.into_iter().collect();

	Ok(LineSegment {
		a: points[0],
		b: points[1],
	})
}

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.map(parse_line)
		.collect::<Result<Vec<LineSegment>, Infallible>>()
		.unwrap()
}

type Solution = usize;

pub fn part_one(segments: &Intermediate) -> Option<Solution> {
	println!("Starting with {} segments", segments.len());

	let segments: Vec<LineSegment> = segments
		.into_iter()
		.filter(|segment| segment.is_horizontal() || segment.is_vertical())
		.copied()
		.collect();

	println!("Ended up with {} segments after filtering", segments.len());

	let mut points: BTreeMap<Point, usize> = BTreeMap::new();

	for segment in segments {
		println!("Segment: {:?}", segment);

		for point in segment.points() {
			println!("  {:?}", point);

			use std::collections::btree_map::Entry;

			match points.entry(point) {
				Entry::Occupied(mut e) => e.insert(e.get() + 1),
				Entry::Vacant(e) => *e.insert(1),
			};
		}
	}

	let overlaps: usize = points.iter().filter(|(&_point, &count)| count >= 2).count();

	Some(overlaps)
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
