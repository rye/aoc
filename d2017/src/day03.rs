use std::collections::BTreeSet;

pub type Intermediate = usize;
pub type Solution = usize;

pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(input.trim().parse()?)
}

daocutil::test_example!(part_one_1, parse, part_one, "1", Some(0));
daocutil::test_example!(part_one_12, parse, part_one, "12", Some(3));
daocutil::test_example!(part_one_23, parse, part_one, "23", Some(2));
daocutil::test_example!(part_one_1024, parse, part_one, "1024", Some(31));

fn gen_grid(size: usize) -> Vec<Option<(i32, i32)>> {
	let mut set: BTreeSet<(i32, i32)> = BTreeSet::new();
	let mut vec = Vec::new();
	let mut dir = (1, 0);

	vec.push(None);

	set.insert((0, 0));
	vec.push(Some((0, 0)));

	while vec.len() - 1 < size {
		let most_recent = vec.last().unwrap().unwrap();
		let next = (most_recent.0 + dir.0, most_recent.1 + dir.1);

		assert!(!set.contains(&next));

		vec.push(Some(next));
		set.insert(next);

		if !set.contains(&(next.0 + -dir.1, next.1 + dir.0)) {
			dir = (-dir.1, dir.0);
		}
	}

	vec
}

#[test]
fn gen_grid_1() {
	assert_eq!(gen_grid(1), vec![None, Some((0, 0))]);
}

#[test]
fn gen_grid_2() {
	assert_eq!(gen_grid(2), vec![None, Some((0, 0)), Some((1, 0))]);
}

#[test]
fn gen_grid_3() {
	assert_eq!(
		gen_grid(3),
		vec![None, Some((0, 0)), Some((1, 0)), Some((1, 1))]
	);
}

#[test]
fn gen_grid_4() {
	assert_eq!(
		gen_grid(4),
		vec![None, Some((0, 0)), Some((1, 0)), Some((1, 1)), Some((0, 1))]
	);
}

#[test]
fn gen_grid_5() {
	assert_eq!(
		gen_grid(5),
		vec![
			None,
			Some((0, 0)),
			Some((1, 0)),
			Some((1, 1)),
			Some((0, 1)),
			Some((-1, 1))
		]
	);
}

#[test]
fn gen_grid_23() {
	assert_eq!(gen_grid(23)[23], Some((0, -2)));
}

pub fn part_one(input: &Intermediate) -> Option<Solution> {
	let grid = gen_grid(*input);

	match grid.get(*input) {
		Some(Some((x, y))) => Some(x.abs() as usize + y.abs() as usize),
		_ => None,
	}
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
