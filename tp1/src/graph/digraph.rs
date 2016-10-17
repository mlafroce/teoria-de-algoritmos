use std::cmp::Ordering;
use std::collections::BTreeSet;

pub type VertexType = usize;
pub type WeightType = i32;

pub struct Digraph {
	vertex_array: Vec<BTreeSet<(VertexType, WeightType)> >,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edge {
	pub src: VertexType,
	pub dst: VertexType,
	pub weight: WeightType,
}

impl Digraph {

	pub fn new(size: VertexType) -> Digraph {
		Digraph {
			vertex_array: vec![BTreeSet::new(); size],
		}
	}

	pub fn vertex_size(&self) -> usize {
		(&self.vertex_array).len()
	}

	pub fn edge_size(&self) -> usize {
		let mut edge_count = 0;
		for edges in &self.vertex_array {
			edge_count += edges.len();
		}
		edge_count
	}

	pub fn adj_edges(&self, vertex: VertexType) -> BTreeSet<Edge> {
		let mut edge_set = BTreeSet::new();
		if let Some(incidence_set) = (&self.vertex_array).get(vertex) {
			for &(dest_v, weight) in incidence_set {
				edge_set.insert(Edge{src: vertex, dst: dest_v, weight: weight});
			}
		}
		edge_set
	}

	pub fn adj_vertexes(&self, vertex: VertexType) -> BTreeSet<VertexType> {
		unimplemented!()
	}

	pub fn add_edge(&mut self, src: VertexType, dest: VertexType) {
		if let Some(dest_set) = (&mut self.vertex_array).get_mut(src) {
			dest_set.insert((dest, 0));
		}
	}

	pub fn add_weighted_edge(&mut self, src: VertexType, dest: VertexType, weight: WeightType) {
		if let Some(dest_set) = (&mut self.vertex_array).get_mut(src) {
			dest_set.insert((dest, weight));
		}
	}

	pub fn iter_edges(&self) {
		unimplemented!()
	}

	pub fn iter_vertexes(&self) -> Vec<VertexType> {
		let range = 0..self.vertex_size();
		range.collect()
	}
}

impl Ord for Edge {
	fn cmp(&self, other: &Edge) -> Ordering {
		let result = self.src.cmp(&other.src);
		if result == Ordering::Equal {
			return self.dst.cmp(&other.dst);
		}
		result
	}
}

impl PartialOrd for Edge {
	fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Edge {
	fn default() -> Edge {
		Edge {src: 0, dst: 0, weight: 0}
	}
}