use std::collections::BTreeSet;
use graph::digraph::Digraph;
use graph::digraph::Edge;
use graph::digraph::VertexType;
use graph::path_finder::PathFinder;
use graph::bfs::Bfs;


#[test]
fn test_vertex_count(){
	let g = Digraph::new(10);
	assert_eq!(g.vertex_size(), 10);
}

#[test]
fn test_edge_count(){
	let mut g = Digraph::new(10);
	g.add_edge(0, 1);
	g.add_edge(0, 2);
	g.add_edge(1, 2);
	g.add_edge(2, 4);
	let edge_count = g.edge_size();
	assert_eq!(edge_count, 4);
}

#[test]
fn test_edge_count_loop(){
	let mut g = Digraph::new(10);
	g.add_edge(0, 0);
	g.add_edge(0, 1);
	g.add_edge(1, 0);
	g.add_edge(1, 1);
	g.add_edge(1, 2);
	let edge_count = g.edge_size();
	assert_eq!(edge_count, 5);
}

#[test]
fn test_add_weighted_edge(){
	let mut g = Digraph::new(3);
	g.add_weighted_edge(0, 0, 5);
	g.add_weighted_edge(0, 1, 2);
	g.add_weighted_edge(1, 0, 10);
	let edge_count = g.edge_size();
	assert_eq!(edge_count, 3);
}

#[test]
fn test_adj_edges(){
	let mut g = Digraph::new(5);
	g.add_edge(0, 0);
	g.add_edge(0, 1);
	g.add_edge(1, 0);
	g.add_edge(1, 1);
	g.add_edge(1, 2);
	g.add_weighted_edge(3, 4, 1);
	assert_eq!(g.adj_edges(0).len(), 2);
	assert_eq!(g.adj_edges(1).len(), 3);
	let edges_from_3 = g.adj_edges(3);
	assert_eq!(edges_from_3.len(), 1);
	// El peso no importa en la comparacíón
	let test_edge = Edge{src: 3, dst: 4, weight: 0};
	assert!(edges_from_3.contains(&test_edge));
	assert!(!edges_from_3.contains(&Edge{src: 10, dst: 4, weight: 0}));
}

#[test]
fn test_adj_vertexes(){
	let mut g = Digraph::new(5);
	g.add_edge(0, 0);
	g.add_edge(0, 1);
	g.add_edge(1, 0);
	g.add_edge(1, 1);
	g.add_edge(1, 2);
	g.add_weighted_edge(3, 4, 1);
	assert_eq!(g.adj_edges(0).len(), 2);
	assert_eq!(g.adj_edges(1).len(), 3);
	let vertexes_to_1:BTreeSet<VertexType> = g.adj_vertexes(1);
	assert_eq!(vertexes_to_1.len(), 2);
	// El peso no importa en la comparacíón
	assert!(vertexes_to_1.contains(&0));
	assert!(vertexes_to_1.contains(&1));
	assert!(!vertexes_to_1.contains(&6));
}

#[test]
fn test_iter_edges(){
	let mut g = Digraph::new(5);
	g.add_edge(0, 0);
	g.add_edge(0, 1);
	assert_eq!(g.iter_edges().len(), 2);
	g.add_edge(1, 0);
	g.add_edge(1, 1);
	g.add_weighted_edge(3, 4, 1);
	assert_eq!(g.iter_edges().len(), 5);
}

#[test]
fn test_bfs_simple() {
	let mut g = Digraph::new(5);
	g.add_weighted_edge(0, 0, 5);
	g.add_weighted_edge(0, 1, 5);
	let mut bfs = Bfs::new(&g);
	let path = bfs.find_path();
	assert_eq!(path.len(), 3);
}