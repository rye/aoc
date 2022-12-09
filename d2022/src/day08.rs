use std::collections::BTreeMap;

pub type Intermediate = (Vec<Vec<u8>>, usize, usize);
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let lines: Vec<Vec<u8>> = input
		.lines()
		.map(|line| {
			line
				.chars()
				.filter_map(|char| char.to_digit(10))
				.filter_map(|u32| u8::try_from(u32).ok())
				.collect()
		})
		.collect();

	let height = lines.len();
	let width = lines[0].len();

	Ok((lines, height, width))
}

const SOUTH: (i8, i8) = (0, 1);
const NORTH: (i8, i8) = (0, -1);
const EAST: (i8, i8) = (1, 0);
const WEST: (i8, i8) = (-1, 0);

// enum Direction {
// 	North,
// 	East,
// 	South,
// 	West,
// }

// fn generate_check_positions_from_pos<'a>(
// 	(pos_x, pos_y): (usize, usize),
// 	range_x: Range<usize>,
// 	range_y: Range<usize>,
// ) -> impl Iterator<Item = (usize, usize)> {
// 	use Direction::*;

// 	[North, East, South, West].iter().flat_map(move |dir| {
// 		let range_x = range_x.clone();
// 		let range_y = range_y.clone();

// 		(1..).scan((), move |_, idx| {
// 			let res: (Option<usize>, Option<usize>) = match *dir {
// 				North => (Some(pos_x), pos_y.checked_sub(idx)),
// 				East => (pos_x.checked_add(idx), Some(pos_y)),
// 				South => (Some(pos_x), pos_x.checked_add(idx)),
// 				West => (pos_x.checked_sub(idx), Some(pos_y)),
// 			};

// 			match res {
// 				(Some(x), Some(y)) if range_x.contains(&x) && range_y.contains(&y) => Some((x, y)),
// 				_ => None,
// 			}
// 		})
// 	})
// }

// #[test]
// fn generate_check_positions_from_pos_ok() {
// 	let (pos_x, pos_y) = (1, 1);
// 	let range_x = 0..3;
// 	let range_y = 0..3;
// 	let mut iter = generate_check_positions_from_pos((pos_x, pos_y), range_x, range_y);

// 	assert_eq!(iter.next(), Some((1, 0)));
// 	assert_eq!(iter.next(), Some((2, 1)));
// 	assert_eq!(iter.next(), Some((1, 2)));
// 	assert_eq!(iter.next(), Some((0, 1)));
// 	assert_eq!(iter.next(), None);
// }

// fn paint_with_visibility(lines: &Vec<Vec<u8>>, visibility: &BTreeMap<(usize, usize), bool>) {
// 	let height = lines.len();
// 	let width = lines[0].len();

// 	let bold_style = nu_ansi_term::Style::new().bold();

// 	for y in 0..height {
// 		for x in 0..width {
// 			let height = lines[y][x];

// 			let visible = match visibility.get(&(x, y)) {
// 				Some(visibility) => *visibility,
// 				_ => false,
// 			};

// 			if visible {
// 				print!("{}", bold_style.paint(format!("{}", height)));
// 			} else {
// 				print!("{}", height);
// 			}
// 		}
// 		println!();
// 	}
// }

fn count_visible(visibility: &BTreeMap<(usize, usize), bool>) -> usize {
	visibility.values().filter(|bool| **bool).count()
}

#[must_use]
pub fn part_one((lines, height, width): &Intermediate) -> Option<Output> {
	let positions = (0..*height).flat_map(|y| (0..*width).map(move |x| (x, y)));

	let mut visibility: BTreeMap<(usize, usize), bool> = BTreeMap::default();

	for (pos_x, pos_y) in positions {
		let height_at_position = lines[pos_y][pos_x];

		let mut visible_in_any_direction: bool = false;

		for (dir_x, dir_y) in [SOUTH, NORTH, EAST, WEST] {
			let mut visible_in_direction: Option<bool> = None;

			let mut c = 1_usize;

			loop {
				let check_x = (pos_x as isize + (dir_x as isize * c as isize)) as usize;
				let check_y = (pos_y as isize + (dir_y as isize * c as isize)) as usize;

				// If this step takes us out of bounds, nothing to check.
				if !(0..*width).contains(&check_x) || !(0..*height).contains(&check_y) {
					break;
				}

				// Check this new position...
				if lines[check_y][check_x] >= height_at_position {
					visible_in_direction = Some(false);
					break;
				}

				c += 1;
			}

			if let Some(visible_in_direction) = visible_in_direction {
				if visible_in_direction {
					visible_in_any_direction = true;
					break;
				} else {
					// Need to check another direction...
				}
			} else {
				visible_in_any_direction = true;
			}
		}

		visibility.insert((pos_x, pos_y), visible_in_any_direction);
	}

	// paint_with_visibility(lines, &visibility);

	Some(count_visible(&visibility))
}

#[test]
fn part_one_example() {
	daocutil::test_example!(
		"30373\n25512\n65332\n33549\n35390",
		part_one,
		parse,
		Some(21)
	);
}

#[must_use]
pub fn part_two((lines, height, width): &Intermediate) -> Option<Output> {
	let positions = (0..*height).flat_map(|y| (0..*width).map(move |x| (x, y)));

	let mut scores: BTreeMap<(usize, usize), usize> = BTreeMap::default();

	for (pos_x, pos_y) in positions {
		let height_at_position = lines[pos_y][pos_x];

		let mut score = 1_usize;

		for (dir_x, dir_y) in [SOUTH, NORTH, EAST, WEST] {
			let mut c = 1_usize;

			loop {
				let check_x = (pos_x as isize + (dir_x as isize * c as isize)) as usize;
				let check_y = (pos_y as isize + (dir_y as isize * c as isize)) as usize;

				// If this step takes us out of bounds, nothing to check.
				if !(0..*width).contains(&check_x) || !(0..*height).contains(&check_y) {
					score *= c - 1;
					break;
				}

				// Check this new position...
				if lines[check_y][check_x] >= height_at_position {
					score *= c;
					break;
				}

				c += 1;
			}
		}

		scores.insert((pos_x, pos_y), score);
	}

	scores.values().max().copied()
}

#[test]
fn part_two_example() {
	daocutil::test_example!(
		"30373\n25512\n65332\n33549\n35390",
		part_two,
		parse,
		Some(8)
	);
}
