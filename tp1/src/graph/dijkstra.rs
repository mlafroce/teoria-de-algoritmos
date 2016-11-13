use graph::digraph::VertexType;
use graph::digraph::WeightType;
use graph::digraph::Digraph;
use graph::path_finder::PathFinder;
use graph::path_finder::PathFinderResult;

use graph::digraph::Edge;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

use std::i32;


pub struct Dijkstra {
  graph: Digraph,
  result: PathFinderResult,
  src: VertexType,
  dst: VertexType,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapminData {
  label: VertexType,
  weight: WeightType,
}


impl Dijkstra {
  pub fn new(graph: &Digraph, src: VertexType, dst: VertexType) -> Dijkstra {
    Dijkstra{graph: graph.clone(), result: PathFinderResult::new(),
      src: src, dst: dst}
  }
}

impl PathFinder for Dijkstra {
  fn find_path(&mut self) {
    let mut distances = vec![i32::MAX; self.graph.size()];
    // Mapa destino -> origen
    let mut visited = HashMap::new();
    let mut min_heap = BinaryHeap::new();
    min_heap.push(HeapminData{label: self.src, weight: 0});
    while let Some(vertex) = min_heap.pop() {
      let adj_edges = self.graph.adj_edges(vertex.label);
      for adj_edge in adj_edges {
        let current_distance = adj_edge.weight + vertex.weight;
         adj_edge.src, adj_edge.dst, adj_edge.weight, vertex.weight, current_distance);
        if distances[adj_edge.dst] > current_distance {
          distances[adj_edge.dst] = current_distance;
          visited.insert(adj_edge.dst, adj_edge.src);
          // Debido a que no puedo modificar el heap por dentro, admito
          // vertices duplicados
          min_heap.push(HeapminData{label: adj_edge.dst, weight: current_distance});
        }
      }
      // Si mis vertices más cercanos ya están más lejos que el destino
      // el destino no se puede optimizar más
      if vertex.weight > distances[self.dst] {
        min_heap.clear();
      }
    }
    self.result = PathFinderResult::new_from_visited(&visited, self.src, self.dst);
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

impl Ord for HeapminData {
  fn cmp(&self, other: &HeapminData) -> Ordering {
    other.weight.cmp(&self.weight)
  }
}

impl PartialOrd for HeapminData {
  fn partial_cmp(&self, other: &HeapminData) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}