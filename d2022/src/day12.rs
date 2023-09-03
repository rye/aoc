use std::collections::{BTreeMap, VecDeque};

use daocutil::neighbors_no_diags;

#[derive(Clone)]
pub struct HeightMap(BTreeMap<(u32, u32), Height>);

pub type Intermediate = (HeightMap, (u32, u32), (u32, u32));
pub type Output = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Height {
	Start,
	Intermediate(u8),
	End,
}

impl From<Height> for char {
	fn from(value: Height) -> Self {
		match value {
			Height::Start => 'S',
			Height::Intermediate(ord) => char::from_u32(u32::from(
				ord + u8::try_from('a').expect("a should fit within a byte"),
			))
			.expect("ascii?!"),
			Height::End => 'E',
		}
	}
}

impl Height {
	#[inline]
	fn is_start(&self) -> bool {
		core::mem::discriminant(self) == core::mem::discriminant(&Self::Start)
	}

	#[inline]
	fn is_end(&self) -> bool {
		core::mem::discriminant(self) == core::mem::discriminant(&Self::End)
	}

	fn can_step_up_to(&self, other: &Height) -> bool {
		match (self, other) {
			(Height::Start, Height::Start) => true,
			(Height::Start, Height::Intermediate(b)) => b <= &1,
			(Height::Start, Height::End) => false,
			(Height::Intermediate(_a), Height::Start) => true,
			(Height::Intermediate(_a), Height::Intermediate(0)) => true,
			(Height::Intermediate(a), Height::Intermediate(b)) => &(a + 1) >= b,
			(Height::Intermediate(a), Height::End) => a >= &25_u8,
			(Height::End, Height::Start) => todo!(),
			(Height::End, Height::Intermediate(_)) => todo!(),
			(Height::End, Height::End) => todo!(),
		}
	}
}

impl TryFrom<char> for Height {
	type Error = anyhow::Error;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'S' => Ok(Self::Start),
			'E' => Ok(Self::End),
			'a'..='z' => Ok(Self::Intermediate(
				u8::try_from(value).expect("a..=z should fit within a single byte")
					- u8::try_from('a').expect("a should fit within a byte"),
			)),
			_ => todo!(),
		}
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let map: BTreeMap<(u32, u32), Height> = input
		.lines()
		.enumerate()
		.flat_map(move |(y, line)| {
			line.chars().enumerate().map(move |(x, c)| {
				(
					(x as u32, y as u32),
					c.try_into().expect("failed to convert to height type"),
				)
			})
		})
		.collect();

	let s_pos = map.iter().find_map(|(location, height)| {
		if height.is_start() {
			Some(*location)
		} else {
			None
		}
	});

	let e_pos = map.iter().find_map(|(location, height)| {
		if height.is_end() {
			Some(*location)
		} else {
			None
		}
	});

	Ok((HeightMap(map), s_pos.unwrap(), e_pos.unwrap()))
}

#[must_use]
pub fn part_one((map, start_pos, end_pos): &Intermediate) -> Option<Output> {
	let mut queue: VecDeque<((u32, u32), usize)> = VecDeque::default();
	let mut visited: BTreeMap<(u32, u32), usize> = BTreeMap::default();

	// Push the first node (the actual last node) into the queue and mark it as visited.
	visited.insert(*end_pos, 0_usize);
	queue.push_back((*end_pos, 0_usize));

	// bfs with working path distance as a carried value.
	while let Some((to_visit, working_path_dist)) = queue.pop_front() {
		// Convert to in-bounds...
		if let (Some(x), Some(y)) = (
			i32::try_from(to_visit.0).ok(),
			i32::try_from(to_visit.1).ok(),
		) {
			for neighbor in neighbors_no_diags(&(x, y)).filter_map(|(x, y)| {
				match (u32::try_from(x).ok(), u32::try_from(y).ok()) {
					(Some(x), Some(y)) if map.0.contains_key(&(x, y)) => Some((x, y)),
					_ => None,
				}
			}) {
				if visited.contains_key(&neighbor) {
					continue;
				}

				let height_at_self = map.0.get(&to_visit);
				let height_at_neighbor = map.0.get(&neighbor);

				let can_step = match (height_at_self, height_at_neighbor) {
					(Some(height_at_self), Some(height_at_neighbor)) => {
						height_at_neighbor.can_step_up_to(height_at_self)
					}
					_ => false,
				};

				if can_step {
					visited.insert(neighbor, working_path_dist + 1);
					queue.push_back((neighbor, working_path_dist + 1));
				}
			}
		}
	}

	visited
		.get(start_pos)
		.map(|usize| u32::try_from(*usize).ok())
		.flatten()
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day12"),
	Some(31)
);

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
