use std::cmp::Ordering;
use std::collections::BTreeSet;

pub type VertexType = usize;
pub type WeightType = i32;

#[derive(Clone)]
pub struct Digraph {
	vertex_array: Vec<BTreeSet<Incidence> >,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Incidence {
	pub dst: VertexType,
	pub weight: WeightType,
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

	/**
	 * Edges from V
	 */
	pub fn adj_edges(&self, vertex: VertexType) -> BTreeSet<Edge> {
		let mut edge_set = BTreeSet::new();
		if let Some(incidence_set) = (&self.vertex_array).get(vertex) {
			for incidence in incidence_set {
				edge_set.insert(Edge{src: vertex, dst: incidence.dst, weight: incidence.weight});
			}
		}
		edge_set
	}

	/**
	 * Edges to V
	 */
	pub fn adj_vertexes(&self, vertex: VertexType) -> BTreeSet<VertexType> {
		let mut vertex_set = BTreeSet::new();
		let dest_aux = Incidence {dst: vertex, weight: 0};
		for (src, incidence_set) in (&self.vertex_array).iter().enumerate() {
			let incidence : Option<&Incidence> = incidence_set.get(&dest_aux);
			if let Some(_) = incidence {
				vertex_set.insert(src);
			} 
		}
		vertex_set
	}

	pub fn add_edge(&mut self, src: VertexType, dst: VertexType) {
		if let Some(dest_set) = (&mut self.vertex_array).get_mut(src) {
			dest_set.insert(Incidence{dst: dst, weight: 0});
		}
	}

	pub fn add_weighted_edge(&mut self, src: VertexType, dst: VertexType, weight: WeightType) {
		if let Some(dest_set) = (&mut self.vertex_array).get_mut(src) {
			dest_set.insert(Incidence{dst: dst, weight: weight});
		}
	}

	pub fn iter_edges(&self) -> BTreeSet<Edge> {
		let mut edges_set = BTreeSet::new();
		for (src, incidence_set) in (&self.vertex_array).iter().enumerate() {
			for incidence in incidence_set {
				let edge = Edge {src: src, dst: incidence.dst, weight: incidence.weight};
				edges_set.insert(edge); 
			}
		}
		edges_set
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


impl Ord for Incidence {
	fn cmp(&self, other: &Incidence) -> Ordering {
		self.dst.cmp(&other.dst)
	}
}

impl PartialOrd for Incidence {
	fn partial_cmp(&self, other: &Incidence) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
