use std::collections::*;
use std::io::{stdin, Read};

use regex::Regex;

use d2020::day07::*;

fn process_color(color: &str) -> String {
	color.replace(" ", "_")
}

fn process_content_spec(content_spec: &str) -> (usize, String) {
	let content_inner_spec = Regex::new("(\\d+) (.+) bags?").unwrap();

	let captures = content_inner_spec.captures(content_spec).unwrap();

	let number = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
	let color = process_color(captures.get(2).unwrap().as_str());

	(number, color)
}

fn process_contents(contents: &str) -> Vec<(usize, String)> {
	match contents {
		"no other bags" => Vec::new(),
		_ => contents
			.split(", ")
			.map(|content| process_content_spec(content))
			.collect(),
	}
}

fn duplicate_color(contents: &(usize, String)) -> Vec<String> {
	std::iter::repeat(contents.1.clone())
		.take(contents.0)
		.collect()
}

fn ruleify(container: String, contents: Vec<(usize, String)>) -> String {
	let colors: Vec<String> = contents
		.iter()
		.map(|tup| duplicate_color(tup))
		.flatten()
		.collect();

	format!("in({}, [{}]).", container, colors.join(", "))
}

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let bag_re = Regex::new("(.+) bags contain (.*).").unwrap();

	for line in data.lines() {
		let caps = bag_re.captures(line).unwrap();

		let color = process_color(caps.get(1).unwrap().as_str());
		let content = process_contents(caps.get(2).unwrap().as_str());

		println!("{}", ruleify(color, content));
	}

	println!(
		"
contains(X,Y) :- in(X,Z), member(Y,Z).

:- table contents/2.
contents(X,Y) :- contains(X,Y).
contents(X,Y) :- contents(X,Z), contents(Z,Y).

expand([],[]).
expand([BAG|BAG_LIST],EXPANSION) :-
    expand(BAG_LIST,LIST_EXPANSION),
    in(BAG,CONTENTS),
    append(CONTENTS,LIST_EXPANSION,EXPANSION).

expand_contents(X,[],[]) :- expand(X,[]).
expand_contents(X,Y,TRACE) :-
    expand(X,Z),
    expand_contents(Z,Y,TRACE1),
    append(Z,TRACE1,TRACE).

size(X,Z) :-
    expand_contents(X,_,TRACE),
    length(TRACE,Z)."
	);
}
