use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone)]
enum Node {
	Start,
	End,
	BigCave(String),
	SmallCave(String),
}

impl From<&str> for Node {
	fn from(str: &str) -> Self {
		// TODO: Intern? Please?
		// Most of the strings are 2-byte strings actually.
		match str {
			"start" => Self::Start,
			"end" => Self::End,
			str if str.as_bytes().iter().all(u8::is_ascii_uppercase) => Self::BigCave(str.to_string()),
			str if str.as_bytes().iter().all(u8::is_ascii_lowercase) => Self::SmallCave(str.to_string()),
			_ => panic!("unrecognized string {str}"),
		}
	}
}

pub struct AdjacencyList(HashMap<Node, HashSet<Node>>);

type Intermediate = AdjacencyList;

// line like start-end
// becomes
// [ (Node::Start, Node::End), (Node::End, Node::Start) ]
//
// (Node::Start, Node::End)
// (Node::End, Node::Start)
// (Node::Start, Node::BigCave("A"))
// (Node::BigCave("A"), Node::Start)
// (Node::Start, Node::SmallCave("a"))
// (Node::SmallCave("a"), Node::Start)

//    a
//    |
//  start -- end
//    |
//    A

pub fn parse(input: &str) -> Intermediate {
	let adjacencies = input
		.lines()
		.flat_map(|line| {
			let pieces: Vec<&str> = line.split('-').collect();

			let (node_a, node_b) = match (pieces.get(0), pieces.get(1)) {
				(Some(&a), Some(&b)) => (Node::from(a), Node::from(b)),
				_ => panic!("failed to find two pieces when splitting on -"),
			};

			vec![(node_a.clone(), node_b.clone()), (node_b, node_a)].into_iter()
		})
		.fold(
			HashMap::new(),
			|mut adjacencies: HashMap<Node, HashSet<Node>>, (node_a, node_b)| {
				adjacencies.entry(node_a).or_default().insert(node_b);
				adjacencies
			},
		);

	AdjacencyList(adjacencies)
}

type Solution = usize;

pub fn part_one(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
