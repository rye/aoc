use {
	core::{convert::Infallible, str::FromStr},
	std::{
		collections::{HashMap, HashSet},
		rc::Rc,
	},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Node {
	Start,
	SmallCave(String),
	LargeCave(String),
	End,
}

impl FromStr for Node {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"start" => Ok(Self::Start),
			"end" => Ok(Self::End),
			s if s.chars().all(|c| c.is_ascii_uppercase()) => Ok(Self::LargeCave(s.to_string())),
			s if s.chars().all(|c| c.is_ascii_lowercase()) => Ok(Self::SmallCave(s.to_string())),
			_ => todo!(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct Graph {
	nodes: HashSet<Rc<Node>>,
	edges: HashMap<Rc<Node>, HashSet<Rc<Node>>>,
}

#[derive(Clone)]
pub struct Edge(Node, Node);

impl FromStr for Edge {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let split: Vec<&str> = s.split('-').collect();

		match split[..] {
			[left, right] => Ok(Self(left.parse()?, right.parse()?)),
			_ => unreachable!(),
		}
	}
}

impl FromIterator<Edge> for Graph {
	fn from_iter<T: IntoIterator<Item = Edge>>(iter: T) -> Self {
		let mut nodes = HashSet::new();

		let mut edges = HashMap::new();

		for edge in iter {
			let (node_left, node_right) = (edge.0, edge.1);

			let node_left = Rc::new(node_left);
			let node_right = Rc::new(node_right);

			// Add edge.
			edges
				.entry(node_left.clone())
				.or_insert(HashSet::default())
				.insert(node_right.clone());

			edges
				.entry(node_right.clone())
				.or_insert(HashSet::default())
				.insert(node_left.clone());

			// Add nodes to node set.
			nodes.insert(node_left);
			nodes.insert(node_right);
		}

		Graph { nodes, edges }
	}
}

pub type Intermediate = Graph;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	input.lines().map(FromStr::from_str).collect()
}

type Solution = usize;

pub fn part_one(_graph: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_graph: &Intermediate) -> Option<Solution> {
	None
}
