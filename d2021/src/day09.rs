#![allow(dead_code, unused)]

use std::collections::{BTreeMap, BTreeSet, HashSet};

pub struct HeightMap {
	points: BTreeMap<(u32, u32), u32>,
}

fn neighbors(x: u32, y: u32) -> impl Iterator<Item = (u32, u32)> {
	let neighbor_offsets: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

	neighbor_offsets
		.into_iter()
		.filter_map(move |(x_offset, y_offset)| {
			if let (Some(x), Some(y)) = (
				u32::try_from(i32::try_from(x).unwrap() + x_offset).ok(),
				u32::try_from(i32::try_from(y).unwrap() + y_offset).ok(),
			) {
				Some((x, y))
			} else {
				None
			}
		})
}

impl HeightMap {
	fn low_points(&self) -> impl Iterator<Item = (&(u32, u32), &u32)> {
		self.points.iter().filter(|((x, y), height)| {
			let neighbors: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

			for (x_offset, y_offset) in neighbors {
				if let (Some(x), Some(y)) = (
					u32::try_from(i32::try_from(*x).unwrap() + x_offset).ok(),
					u32::try_from(i32::try_from(*y).unwrap() + y_offset).ok(),
				) {
					if let Some(neighbor_value) = self.points.get(&(x, y)) {
						if neighbor_value <= height {
							return false;
						}
					}
				}
			}

			return true;
		})
	}

	fn height_at(&self, (x, y): (u32, u32)) -> u32 {
		match self.points.get(&(x, y)) {
			Some(height) => *height,
			None => 9,
		}
	}

	fn count(&self) -> usize {
		self.points.len()
	}
}

pub type Intermediate = HeightMap;

pub fn parse(points: &str) -> Result<Intermediate, core::convert::Infallible> {
	let points = points
		.lines()
		.enumerate()
		.map(|(y, line)| {
			line
				.chars()
				.enumerate()
				.map(|(x, c)| (x, c))
				.map(move |(x, c)| {
					(
						(x.try_into().unwrap(), y.try_into().unwrap()),
						c.to_digit(10).unwrap(),
					)
				})
		})
		.flatten()
		.collect();

	Ok(HeightMap { points })
}

type Solution = u32;

pub fn part_one(height_map: &Intermediate) -> Option<Solution> {
	let low_points = height_map.low_points().map(|(_, height)| height);

	let risk_levels = low_points.map(|height| height + 1).sum();

	Some(risk_levels)
}

pub fn part_two(height_map: &Intermediate) -> Option<Solution> {
	use std::collections::{
		hash_map::Entry,
		{BTreeMap, BTreeSet}, {HashMap, HashSet, VecDeque},
	};

	let mut seen: HashSet<(u32, u32)> = HashSet::new();

	// Map from low point -> which points are in the basin.
	let mut basin_points: HashMap<(u32, u32), HashSet<(u32, u32)>> = HashMap::new();

	// Map from point -> which basin it belongs to.
	let mut points_to_basin: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

	let mut queue: VecDeque<(u32, u32)> = VecDeque::new();

	for ((x, y), h) in height_map.low_points() {
		let mut basic = HashSet::new();
		basic.insert((*x, *y));
		basin_points.insert((*x, *y), basic);

		points_to_basin.insert((*x, *y), (*x, *y));

		queue.push_back((*x, *y));
	}

	while let Some((x, y)) = queue.pop_front() {
		seen.insert((x, y));

		let basin_origin: (u32, u32) = *points_to_basin
			.get(&(x, y))
			.expect("basin point with no origin");

		let self_height = height_map.height_at((x, y));

		// Remove a point from the queue.

		for (nx, ny) in neighbors(x, y) {
			let neighbor_height = height_map.height_at((nx, ny));

			// If the neighbor's height is 9, it doesn't count.
			if neighbor_height == 9 {
				continue;
			} else if neighbor_height > self_height {
				// Add the neighbor to the queue to explore.
				queue.push_back((nx, ny));
				points_to_basin.insert((nx, ny), basin_origin);

				match basin_points.entry(basin_origin) {
					Entry::Occupied(mut e) => e.get_mut().insert((nx, ny)),
					Entry::Vacant(e) => panic!("no basin entry D:"),
				};
			}

			// If the neighbor is lower or equal, don't add it to the queue.
		}
	}

	let basin_sizes = basin_points
		.iter()
		.map(|((x, y), set)| (set.len(), (*x, *y)))
		.collect::<BTreeSet<(usize, (u32, u32))>>();

	let top_3_sizes = basin_sizes
		.iter()
		.rev()
		.take(3)
		.map(|(sz, _)| *sz)
		.collect::<Vec<usize>>();

	Some(
		top_3_sizes
			.iter()
			.fold(1_u32, |mut acc, sz| acc * (*sz as u32)),
	)
}

#[test]
fn part_two_example_0() {
	let string = "2199943210
3987894921
9856789892
8767896789
9899965678";
	let height_map = parse(string).expect("failed to parse");
}
