use graph::digraph::VertexType;
use graph::digraph::Edge;
use std::collections::HashMap;
use std::collections::HashSet;

pub type DistanceType = usize;

pub struct PathFinderResult {
	pub path: Vec<VertexType>,
	pub edge_map: HashMap<VertexType, Edge>,
	pub visited_set: HashSet<VertexType>,
}

impl PathFinderResult {
	pub fn new() -> PathFinderResult {
		PathFinderResult {
			path: vec![],
			edge_map: HashMap::new(),
			visited_set: HashSet::new()
		}
	}

	pub fn new_from_visited(&self, visited: &HashMap<VertexType, VertexType>, dst: VertexType) {
		let mut last = dest;
		while let Some(src) = visited.get(last) {
			self.path.push_front(last);
			last = src;
		}
		self.path.push_front(last);
	}
}


pub trait PathFinder {

	/**
	 * Mejorar esta parte del trait
	 */
	fn find_path(&self);


	/**
	 * Cantidad de aristas que hay que recorrer para encontrar el destino,
	 * recorriendo el camino óptimo encontrado.
	 */
	fn distance(&self) -> DistanceType {
		self.get_path().len()
	}

	/**
	 * Devuelve el arista al vértice vertex, dentro del camino encontra{do
	 */
	fn edge_to(&self, vertex: VertexType) -> Edge {
		let some_edge = self.get_edge_map().get(&vertex);
		*some_edge.unwrap()
	}

	fn visited(&self, vertex: VertexType) -> bool {
		self.get_visited_set().contains(&vertex)
	}

	fn get_path(&self) -> &Vec<VertexType>;

	fn get_edge_map(&self) -> &HashMap<VertexType, Edge>;

	fn get_visited_set(&self) -> &HashSet<VertexType>;
}