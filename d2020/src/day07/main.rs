use std::io::{stdin, Read};

use regex::Regex;

use d2020::day07::*;

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
