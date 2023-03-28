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

struct GridIter {
	set: BTreeSet<(i32, i32)>,
	last: (i32, i32),
	dir: (i32, i32),
}

impl Default for GridIter {
	fn default() -> Self {
		let mut set = BTreeSet::new();
		set.insert((0, 0));
		Self {
			set,
			last: (0, 0),
			dir: (1, 0),
		}
	}
}

impl Iterator for GridIter {
	type Item = (i32, i32);

	fn next(&mut self) -> Option<Self::Item> {
		let cur = self.last;
		let next = (self.last.0 + self.dir.0, self.last.1 + self.dir.1);
		assert!(!self.set.contains(&next));

		self.set.insert(next);
		self.last = next;

		if !self
			.set
			.contains(&(next.0 + -self.dir.1, next.1 + self.dir.0))
		{
			self.dir = (-self.dir.1, self.dir.0);
		}

		Some(cur)
	}
}

#[test]
fn gen_grid_iter() {
	let mut gen_grid = GridIter::default();
	assert_eq!(gen_grid.next(), Some((0, 0)));
	assert_eq!(gen_grid.next(), Some((1, 0)));
	assert_eq!(gen_grid.next(), Some((1, 1)));
	assert_eq!(gen_grid.next(), Some((0, 1)));
	assert_eq!(gen_grid.next(), Some((-1, 1)));
	assert_eq!(gen_grid.next(), Some((-1, 0)));
	assert_eq!(gen_grid.next(), Some((-1, -1)));
	assert_eq!(gen_grid.next(), Some((0, -1)));
}

pub fn part_one(input: &Intermediate) -> Option<Solution> {
	let nth = GridIter::default().nth(*input - 1);

	match nth {
		Some((x, y)) => Some(x.abs() as usize + y.abs() as usize),
		_ => None,
	}
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
