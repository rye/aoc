use std::collections::BTreeSet;
use std::io::{stdin, Read};

use d2020::day07::*;

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let groups: Vec<&str> = data.split("\n\n").collect();

	{
		println!("Part One: {:?}", ());
	}

	{
		println!("Part Two: {:?}", ());
	}
}
