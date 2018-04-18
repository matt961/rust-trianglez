use std::collections::hash_map::Keys;
use std::collections::hash_set::Iter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use rayon::prelude::*;
use rayon::slice;
use rayon::collections::hash_set;

type AdjacencyList<N> = HashSet<N>;

pub trait Node: Hash + Eq + PartialEq + Clone + Send + Sync {}

impl<T> Node for T where T: Hash + Eq + PartialEq + Clone + Send + Sync {}

pub struct Graph<N: Node>
{
	adj_list: HashMap<N, AdjacencyList<N>>,
}

impl<N> Graph<N>
where
	N: Node
{
	pub fn new() -> Graph<N> {
		Graph {
			adj_list: HashMap::new(),
		}
	}

	pub fn add_edge(&mut self, a: N, b: N) -> bool {
		if a != b {
			self.adj_list
				.entry(a.clone())
				.or_insert(AdjacencyList::new())
				.insert(b.clone());
			self.adj_list
				.entry(b.clone())
				.or_insert(AdjacencyList::new())
				.insert(a.clone());
			true
		} else {
			false
		}
	}

	pub fn nodes_par(&self) -> Vec<&N> {
		self.adj_list.keys().collect()
	}

	pub fn nodes(&self) -> Keys<N, AdjacencyList<N>> {
		self.adj_list.keys()
	}

	pub fn contains_edge(&self, a: &N, b: &N) -> bool {
		self.adj_list.get(a).map_or(false, |list| list.contains(b))
	}

	pub fn node_count(&self) -> usize {
		self.adj_list.len()
	}

	pub fn edge_count(&self) -> usize {
		let count: usize = self.adj_list.values().map(|list| list.len()).sum();
		count / 2_usize
	}

	pub fn neighbors(&self, a: &N) -> Option<impl Iterator<Item = &N>> {
		Some(self.adj_list.get(a)?.iter())
	}

	pub fn degree(&self, a: &N) -> usize {
		self.adj_list.get(a).map_or(0, |list| list.len())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_graph() {
		let mut my_g = Graph::new();
		my_g.add_edge(1, 2);
		my_g.add_edge(2, 3);
		my_g.add_edge(3, 1);
		my_g.add_edge(4, 1);
		assert_eq!(my_g.contains_edge(&1, &2), true);
		assert_eq!(my_g.contains_edge(&2, &3), true);
		assert_eq!(my_g.contains_edge(&3, &1), true);
		assert_eq!(my_g.contains_edge(&4, &2), false);
		my_g.nodes().all(|node| [1, 2, 3, 4].contains(node));
		assert!(
			my_g.neighbors(&1)
				.map_or(false, |mut neighbors| neighbors.all(|neighbor| [3, 4, 2].contains(neighbor)))
		);
		assert!(
			my_g.neighbors(&4)
				.map_or(false, |mut neighbors| neighbors.all(|neighbor| [1].contains(neighbor)))
		);
		assert_eq!(my_g.neighbors(&5).map_or(false, move |_| true), false);
	}
}
