use {
	core::iter::empty,
	std::{
		collections::{HashMap, HashSet},
		rc::Rc,
	},
};

use super::{edge::Edge, node::Node};

#[derive(Clone, Debug)]
pub struct Graph {
	nodes: HashSet<Rc<Node>>,
	edges: HashMap<Rc<Node>, HashSet<Rc<Node>>>,
}

impl Graph {
	pub fn to_graphviz(&self) -> String {
		let mut string: String = String::new();

		for (left, right_set) in &self.edges {
			for right in right_set {
				string.push_str(&format!("{} -- {}\n", left, right));
			}
		}

		string
	}

	pub fn nodes_from(&self, node: &Node) -> Box<dyn Iterator<Item = &Node> + '_> {
		match self.edges.get(node) {
			Some(set) => Box::new(set.iter().map(Rc::as_ref)),
			None => Box::new(empty()),
		}
	}
}

impl FromIterator<Edge> for Graph {
	fn from_iter<T: IntoIterator<Item = Edge>>(iter: T) -> Self {
		let mut nodes = HashSet::new();

		let mut edges = HashMap::new();

		for edge in iter {
			let (node_left, node_right) = edge.into();

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
