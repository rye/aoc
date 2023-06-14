use core::{cell::Cell, str::FromStr};
use std::{
	collections::{BTreeMap, VecDeque},
	sync::OnceLock,
};

use regex::Regex;

pub type Intermediate = BTreeMap<u8, Monkey>;
pub type Output = u32;

const PARSE_RE_STR: &str = r"(?s)^Monkey (?P<id>\d+):$\n^\s+Starting items: (?P<si>[\d, ]+)$\n^\s+Operation: new = (?P<oprd1>\b[\w\d]+\b) (?P<oprtr>[*+]) (?P<oprd2>\b[\w\d]+\b)$\n^\s+Test: divisible by (?P<divisor>\d+)$\n^\s+If true: throw to monkey (?P<true_dest>\d+).*?$\n^\s+If false: throw to monkey (?P<false_dest>\d+)$";

static PARSE_RE: OnceLock<Regex> = OnceLock::new();

#[derive(Clone, Copy, Debug)]
enum Operation {
	Add(u32),
	Mul(u32),
	Square,
}

#[derive(Clone, Debug)]
pub struct Monkey {
	id: u8,
	items: VecDeque<u32>,
	operation: Operation,
	divisor: u32,
	true_dest: u8,
	false_dest: u8,
	inspect_counter: usize,
}

impl FromStr for Monkey {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> anyhow::Result<Self> {
		let regex = PARSE_RE.get_or_init(|| {
			regex::RegexBuilder::new(PARSE_RE_STR)
				.multi_line(true)
				.dot_matches_new_line(true)
				.build()
				.expect("failed to compile regex")
		});

		let captures = regex.captures(s).expect("no captures for monkey spec");

		let id: u8 = captures
			.name("id")
			.map(|mtch| mtch.as_str())
			.map(str::parse)
			.expect("expected to get id")
			.expect("expected to be able to parse id as u8");

		let items: VecDeque<u32> = captures
			.name("si")
			.expect("no match for starting items")
			.as_str()
			.split(", ")
			.map(str::parse)
			.collect::<Result<Vec<_>, _>>()?
			.into();

		let operation: Operation = match (
			captures.name("oprd1").map(|m| m.as_str()),
			captures.name("oprtr").map(|m| m.as_str()),
			captures.name("oprd2").map(|m| m.as_str()),
		) {
			(Some("old"), Some("*"), Some("old")) => Operation::Square,
			(Some("old"), Some("*"), Some(k)) => {
				Operation::Mul(k.parse().expect("failed to parse second operator"))
			}
			(Some("old"), Some("+"), Some(k)) => {
				Operation::Add(k.parse().expect("failed to parse second operator"))
			}
			_ => unreachable!(),
		};
		let divisor: u32 = captures
			.name("divisor")
			.expect("did not get match for divisor")
			.as_str()
			.parse()?;
		let true_dest: u8 = captures
			.name("true_dest")
			.expect("did not get match for true dest")
			.as_str()
			.parse()?;
		let false_dest: u8 = captures
			.name("false_dest")
			.expect("did not get match for false dest")
			.as_str()
			.parse()?;

		let monkey = Monkey {
			id,
			items,
			operation,
			divisor,
			true_dest,
			false_dest,
			inspect_counter: 0_usize,
		};

		Ok(monkey)
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	input
		.split("\n\n")
		.map(str::parse)
		.map(|maybe_monkey| maybe_monkey.map(|monkey: Monkey| (monkey.id, monkey)))
		.collect::<Result<BTreeMap<u8, Monkey>, _>>()
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
