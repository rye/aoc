use {core::convert::Infallible, std::collections::BTreeMap};

#[derive(Debug)]
enum Line<'a> {
	CommandLine(&'a str),
	DirLine(&'a str),
	FileLine(usize, &'a str),
}

impl<'a> TryFrom<&'a str> for Line<'a> {
	type Error = Infallible;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		// Lines come in three forms:
		//
		// - $ <cmd> <args...>
		// - dir <directory to explore>
		// - <size> filename

		if &value[0..2] == "$ " {
			Ok(Self::CommandLine(&value[2..]))
		} else if &value[0..4] == "dir " {
			Ok(Self::DirLine(&value[4..]))
		} else {
			let mut split = value.split(' ');

			match (split.next().map(str::parse), split.next()) {
				(Some(Ok(sz)), Some(name)) => Ok(Self::FileLine(sz, name)),
				_ => unreachable!(),
			}
		}
	}
}

pub struct DirectoryTree(BTreeMap<Vec<String>, Option<usize>>);

pub type Intermediate = DirectoryTree;
pub type Output = usize;

fn canonicalize(slice: &[String]) -> Vec<String> {
	let mut working: Vec<Option<String>> = slice.iter().cloned().map(|string| Some(string)).collect();

	loop {
		if let Some(idx) = working.iter().position(|c| match c {
			Some(reldir) if reldir == ".." => true,
			Some(_reldir) => false,
			None => false,
		}) {
			// Take the working slice from [ ..., "aa", "a", "..", "file", ... ] to
			//                             [ ..., "aa",            "file", ... ]

			working[idx - 1] = None;
			working[idx] = None;
		} else {
			break;
		}
	}

	working.into_iter().filter_map(|c| c).collect()
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let lines: Vec<Line> = input
		.lines()
		.map(TryFrom::try_from)
		.collect::<Result<Vec<Line>, _>>()?;

	let mut cwd: Option<Vec<String>> = None;

	let mut tree = BTreeMap::default();

	for line in lines {
		match line {
			Line::CommandLine(command) => {
				let parts: Vec<&str> = command.split(' ').collect();

				match parts[0] {
					"cd" => match (&mut cwd, parts[1]) {
						(None, "/") => {
							cwd.replace(vec!["".to_string()]);

							tree.insert(vec!["".to_string()], None);
						}
						(Some(cwd), reldir) => {
							let mut tmp = cwd.clone();
							tmp.push(reldir.to_string());
							*cwd = canonicalize(&tmp);
						}
						(None, _) => unreachable!(),
					},
					"ls" => {}
					_ => unreachable!(),
				}
			}
			Line::DirLine(dirname) => match &cwd {
				Some(cwd) => {
					let mut key = cwd.clone();
					key.push(dirname.to_string());
					tree.insert(canonicalize(&key), None);
				}
				None => todo!(),
			},
			Line::FileLine(size, filename) => match &cwd {
				Some(cwd) => {
					let mut key = cwd.clone();
					key.push(filename.to_string());
					tree.insert(canonicalize(&key), Some(size));
				}
				None => todo!(),
			},
		};
	}

	Ok(DirectoryTree(tree))
}

#[test]
fn parse_ok() {
	let example = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

	let result = parse(example).expect("failed to parse");

	// - / (dir)
	//   - a (dir)
	//     - e (dir)
	//       - i (file, size=584)
	//     - f (file, size=29116)
	//     - g (file, size=2557)
	//     - h.lst (file, size=62596)
	//   - b.txt (file, size=14848514)
	//   - c.dat (file, size=8504156)
	//   - d (dir)
	//     - j (file, size=4060174)
	//     - d.log (file, size=8033020)
	//     - d.ext (file, size=5626152)
	//     - k (file, size=7214296)

	assert_eq!(
		vec![
			(vec!["".to_string()], None),
			(vec!["".to_string(), "a".to_string()], None),
			(vec!["".to_string(), "a".to_string(), "e".to_string()], None),
			(
				vec![
					"".to_string(),
					"a".to_string(),
					"e".to_string(),
					"i".to_string()
				],
				Some(584)
			),
			(
				vec!["".to_string(), "a".to_string(), "f".to_string()],
				Some(29116)
			),
			(
				vec!["".to_string(), "a".to_string(), "g".to_string()],
				Some(2557)
			),
			(
				vec!["".to_string(), "a".to_string(), "h.lst".to_string()],
				Some(62596)
			),
			(vec!["".to_string(), "b.txt".to_string()], Some(14848514)),
			(vec!["".to_string(), "c.dat".to_string()], Some(8504156)),
			(vec!["".to_string(), "d".to_string()], None),
			(
				vec!["".to_string(), "d".to_string(), "j".to_string()],
				Some(4060174)
			),
			(
				vec!["".to_string(), "d".to_string(), "d.log".to_string()],
				Some(8033020)
			),
			(
				vec!["".to_string(), "d".to_string(), "d.ext".to_string()],
				Some(5626152)
			),
			(
				vec!["".to_string(), "d".to_string(), "k".to_string()],
				Some(7214296)
			),
		]
		.into_iter()
		.collect::<BTreeMap<Vec<String>, Option<usize>>>(),
		result.0
	);
}

#[must_use]
pub fn part_one(DirectoryTree(tree): &Intermediate) -> Option<Output> {
	let working_tree = tree.clone();

	let mut directory_sizes: BTreeMap<Vec<String>, Option<usize>> = BTreeMap::new();

	for (name, size) in &working_tree {
		if size.is_none() {
			directory_sizes.insert(name.clone(), None);
		}
	}

	for (directory_to_populate, dir_size) in &mut directory_sizes {
		*dir_size = Some(0_usize);

		for (entry, size) in &working_tree {
			if let Some(size) = size {
				if entry.len() < directory_to_populate.len() {
					continue;
				}
				if &entry[0..directory_to_populate.len()] != directory_to_populate.as_slice() {
					continue;
				} else {
					match dir_size.as_mut() {
						Some(x) => {
							*x += size;
						}
						None => panic!("asdf"),
					}
				}
			} else {
				continue;
			}
		}
	}

	Some(
		directory_sizes
			.iter()
			.filter_map(|(_dir, sz)| match sz {
				Some(size) if *size <= 100_000 => Some(size),
				Some(_size) => None,
				None => None,
			})
			.sum(),
	)
}

#[must_use]
pub fn part_two(DirectoryTree(tree): &Intermediate) -> Option<Output> {
	let working_tree = tree.clone();

	const FS_SIZE: usize = 70_000_000;
	const MIN_UNUSED_SIZE: usize = 30_000_000;

	let mut directory_sizes: BTreeMap<Vec<String>, usize> = BTreeMap::new();

	for (name, size) in &working_tree {
		if size.is_none() {
			directory_sizes.insert(name.clone(), 0_usize);
		}
	}

	for (directory_to_populate, dir_size) in &mut directory_sizes {
		for (entry, size) in &working_tree {
			if let Some(size) = size {
				if entry.len() < directory_to_populate.len() {
					continue;
				}
				if &entry[0..directory_to_populate.len()] != directory_to_populate.as_slice() {
					continue;
				} else {
					*dir_size += size;
				}
			} else {
				continue;
			}
		}
	}

	let outer_directory_size: usize = *directory_sizes
		.get(&vec!["".to_string()])
		.expect("no size for root directory?!");

	let mut sizes_to_directories: BTreeMap<usize, Vec<Vec<String>>> = BTreeMap::new();

	for (directory, size) in directory_sizes {
		sizes_to_directories
			.entry(size)
			.or_default()
			.push(directory);
	}

	let current_avail = FS_SIZE - outer_directory_size;

	let min_avail = MIN_UNUSED_SIZE;

	if current_avail < min_avail {
		let size_to_free_up = min_avail - current_avail;

		let sizes: Vec<usize> = sizes_to_directories.keys().copied().collect();

		sizes.iter().find(|size| **size > size_to_free_up).copied()
	} else {
		None
	}
}
