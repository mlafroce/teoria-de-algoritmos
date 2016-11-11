use graph::digraph::VertexType;
use graph::digraph::Digraph;
use graph::path_finder::PathFinder;
use graph::path_finder::PathFinderResult;

use graph::digraph::Edge;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;


pub struct Bfs {
	graph: Digraph,
	result: PathFinderResult,
	src: VertexType,
	dst: VertexType,
}

impl Bfs {
	pub fn new(graph: &Digraph, src: VertexType, dst: VertexType) -> Bfs {
		Bfs{graph: graph.clone(), result: PathFinderResult::new(),
			src: src, dst: dst}
	}
}

impl PathFinder for Bfs {
	fn find_path(&mut self) {
		let mut heap = VecDeque::new();
		// Mapa destino -> origen
		let mut visited = HashMap::new();
		let mut found = false;
		// Este algoritmo ignora pesos
		heap.push_back(Edge{src: self.src, dst: self.src, weight: 0});
		while let Some(edge) = heap.pop_front() {
			let adj_edges = self.graph.adj_edges(edge.dst);
			for adj_edge in adj_edges {
				if !visited.contains_key(&adj_edge.dst) {
					visited.insert(adj_edge.dst, adj_edge.src);
					if adj_edge.dst == self.dst {
						found = true;
						println!("found");
					}
					heap.push_back(Edge{src: adj_edge.src, dst: adj_edge.dst, weight: 0});
				}
			}
			// Mejorar esta condición, no se como ponerlo al lado del let,
			if found {
				heap.clear();
			}
		}
	self.result = PathFinderResult::new_from_visited(&visited, self.dst);
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
