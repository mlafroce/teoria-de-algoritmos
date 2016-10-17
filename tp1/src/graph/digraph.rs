use std::collections::BTreeSet;

pub type VertexType = usize;
pub type WeightType = i32;

pub struct Digraph {
	vertex_array: Vec<BTreeSet<(VertexType, WeightType)> >,
}

pub struct Edge {
	src: VertexType,
	dst: VertexType,
	weight: WeightType,
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

	pub fn adj_edges(&self) -> BTreeSet<Edge> {
		unimplemented!()
	}

	pub fn adj_vertexes(&self) -> BTreeSet<VertexType> {
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

	pub fn iter_vertexes(&self) {
		unimplemented!()
	}

	pub fn iter_edges(&self) {
		unimplemented!()
	}
}
