#![allow(dead_code, unused)]

use std::collections::{BTreeMap, BTreeSet};

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
}

type Intermediate = HeightMap;

pub fn parse(points: &str) -> Intermediate {
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

	HeightMap { points }
}

type Solution = u32;

pub fn part_one(height_map: &Intermediate) -> Option<Solution> {
	let low_points = height_map.low_points().map(|(_, height)| height);

	let risk_levels = low_points.map(|height| height + 1).sum();

	Some(risk_levels)
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
