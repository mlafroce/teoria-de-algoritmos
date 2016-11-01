use graph::digraph::VertexType;
use graph::digraph::Edge;
use std::collections::HashMap;
use std::collections::HashSet;

pub type DistanceType = usize;

struct PathFinder {
  path: Vec<VertexType>,
  edge_map: HashMap<VertexType, Edge>,
  visited_set: HashSet<VertexType>,
}

impl PathFinder {

  fn find_path(&self) -> Vec<VertexType> {
    vec![]
  }

  fn distance(&self) -> DistanceType {
    *&(self.path).len()
  }

  fn edge_to(&self, vertex: VertexType) -> Edge {
    let some_edge = &(self.edge_map).get(&vertex);
    *some_edge.unwrap()
  }

  fn visited(&self, vertex: VertexType) -> bool {
    *&(self.visited_set).contains(&vertex)
  }

}