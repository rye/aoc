use std::{path::PathBuf, process::Command};

fn main() {
	let mut program = PathBuf::from(PathBuf::from(file!()).parent().unwrap());
	program.push("..");
	program.push("day08.py");

	let status = Command::new(program)
		.env("RYE_AOC_SUBPROC", "1")
		.env("RYE_AOC_YEAR", "2015")
		.env("RYE_AOC_DAY", "08")
		.env("RYE_AOC_PART_ONE", "1")
		.env("RYE_AOC_PART_TWO", "1")
		.status()
		.expect("failed to execute day08.py");

	if !status.success() {
		eprintln!("Subprocess exited with status {}", status);
	}
}
