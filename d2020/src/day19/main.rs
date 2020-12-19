#![allow(unused_imports)]

use std::io::{stdin, Read};
use std::{collections::*, str::FromStr};

use regex::Regex;

use d2020::{day19::*, *};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
	SingleChar(char),
	Compound(Vec<Vec<usize>>),
}

impl FromStr for Rule {
	type Err = ();
	fn from_str(str: &str) -> Result<Self, ()> {
		let chars: Vec<char> = str.chars().collect();

		match chars[0] {
			'"' => Ok(Self::SingleChar(chars[1])),
			_ => {
				let groups: Vec<Vec<usize>> = str
					.split(" | ")
					.map(|group| {
						group
							.split(" ")
							.map(|n| n.parse::<usize>().unwrap())
							.collect()
					})
					.collect();

				Ok(Self::Compound(groups))
			}
		}
	}
}

#[derive(Debug)]
struct RuleSet {
	rules: Vec<Rule>,
}

impl FromStr for RuleSet {
	type Err = ();

	fn from_str(str: &str) -> Result<Self, ()> {
		let split: Vec<(usize, &str)> = str
			.lines()
			.map(|line| {
				let mut split = line.split(": ");

				(
					split.next().unwrap().parse::<usize>().unwrap(),
					split.next().unwrap(),
				)
			})
			.collect();

		let size: usize = *split.iter().map(|(idx, _)| idx).max().unwrap();

		let mut rules: Vec<Option<Rule>> = Vec::new();
		rules.resize(size + 1_usize, None);

		for (idx, str) in split {
			let rule: Rule = str.parse().unwrap();
			let idx: usize = idx;

			debug_assert!(idx < rules.len());
			assert_eq!(rules[idx], None);

			rules[idx] = Some(rule);
		}

		let rules: Vec<Option<Rule>> = rules;

		let rules: Vec<Rule> = rules.into_iter().filter_map(|x| x).collect();

		assert_eq!(rules.len(), size + 1);

		Ok(RuleSet { rules })
	}
}

impl RuleSet {
	fn message_matches_rule(&self, message: &Message, rule_idx: usize) -> bool {
		self
			.message_matches_rule_inner(message, rule_idx, 0_usize)
			.0
	}

	fn message_matches_rule_inner(
		&self,
		message: &Message,
		rule_idx: usize,
		head: usize,
	) -> (bool, usize) {
		println!("MMRI: {}, {}, {}", message.0, rule_idx, head);

		let rule: &Rule = &self.rules[rule_idx];

		match rule {
			Rule::SingleChar(c) => {
				println!("{:?} == {}?", message.0.chars().nth(head), c);
				(message.0.chars().nth(head) == Some(*c), 1)
			}
			Rule::Compound(groups) => {
				let result = groups
					.iter()
					.map(|group: &Vec<usize>| (group, group.len()))
					.map(|(group, len)| -> (bool, usize) {
						let mut inner_head = head;
						let mut all_matched = true;

						for (idx, rule_idx) in group.iter().enumerate() {
							dbg!(idx, rule_idx, head, inner_head);

							if let (true, size) =
								self.message_matches_rule_inner(message, *rule_idx, inner_head + idx)
							{
								inner_head += size;
							} else {
								all_matched = false;
								break;
							}
						}

						(all_matched, len + inner_head)
					})
					.find(|(b, _)| *b);

				if let Some((b, u)) = result {
					(b, u)
				} else {
					(false, 0)
				}
			}
		}
	}
}

#[derive(Debug)]
struct Message<'x>(&'x str);

fn main() {
	let data: String = string_from(stdin()).unwrap();

	let (rules, messages): (RuleSet, Vec<Message>) = {
		let split: Vec<&str> = data.split("\n\n").collect();

		let messages: Vec<Message> = split[1].lines().map(|line| Message(line)).collect();

		(split[0].parse().unwrap(), messages)
	};

	{
		let count = messages
			.iter()
			.filter(|message| rules.message_matches_rule(message, 0_usize))
			.count();

		println!("Part One: {:?}", count);
	}

	{
		println!("Part Two: {:?}", ());
	}
}
