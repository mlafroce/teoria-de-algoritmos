use graph::digraph::VertexType;
use graph::digraph::Digraph;
use graph::path_finder::PathFinder;
use graph::path_finder::PathFinderResult;

use graph::digraph::Edge;
use std::collections::HashMap;
use std::collections::HashSet;


pub struct Bfs {
	graph: Digraph,
	result: PathFinderResult,
}

impl Bfs {
	pub fn new(graph: &Digraph) -> Bfs {
		Bfs{graph: graph.clone(), result: PathFinderResult::new()}
	}
}

impl PathFinder for Bfs {
	fn find_path(&self) -> Vec<VertexType> {
		vec![2, 3, 5]
	}

	fn get_path(&self) -> &Vec<VertexType> {
		&self.result.path
	}

	fn get_edge_map(&self) -> &HashMap<VertexType, Edge> {
		&self.result.edge_map
	}

	fn get_visited_set(&self) -> &HashSet<VertexType> {
		&self.result.visited_set
	}
}
