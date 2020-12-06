use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

use d2020::day01::*;
use d2020::Lines;

fn main() {
	let stdin = stdin();
	let stdin = stdin.lock();

	let list: BTreeSet<i64> = (Lines::from(stdin.lines())).into();

	let pair = find_pair(&list, &2020_i64);

	if let Some((a, b)) = pair {
		println!("Part One: {}", a * b);
	}

	let triple = find_triple(&list, &2020_i64);

	if let Some((a, b, c)) = triple {
		println!("Part Two: {}", a * b * c);
	}
}
