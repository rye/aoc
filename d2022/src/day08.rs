use {core::ops::Range, std::collections::BTreeMap};

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

fn paint_with_visibility(lines: &Vec<Vec<u8>>, visibility: &BTreeMap<(usize, usize), bool>) {
	let height = lines.len();
	let width = lines[0].len();

	let bold_style = nu_ansi_term::Style::new().bold();

	for y in 0..height {
		for x in 0..width {
			let height = lines[y][x];

			let visible = match visibility.get(&(x, y)) {
				Some(visibility) => *visibility,
				_ => false,
			};

			if visible {
				print!("{}", bold_style.paint(format!("{}", height)));
			} else {
				print!("{}", height);
			}
		}
		println!();
	}
}

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
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
