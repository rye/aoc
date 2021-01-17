use std::collections::BTreeSet;

use d2020::day01::*;

type Intermediate = BTreeSet<i64>;
type Solution = i64;

fn parse(data: &str) -> Intermediate {
	data
		.lines()
		.map(|line| line.parse::<i64>().expect("malformed input"))
		.collect()
}

fn part_one(list: &Intermediate) -> Option<Solution> {
	find_pair(&list, &2020_i64).map(|(a, b): (i64, i64)| a * b)
}

fn part_two(list: &Intermediate) -> Option<Solution> {
	find_triple(&list, &2020_i64).map(|(a, b, c): (i64, i64, i64)| a * b * c)
}

d2020::day_solver!(Intermediate, Solution, parse, part_one, part_two);
