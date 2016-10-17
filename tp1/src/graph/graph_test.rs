use graph::digraph::Digraph;

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
