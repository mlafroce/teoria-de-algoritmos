use graph::digraph::Digraph;
use graph::path_finder::PathFinder;
use graph::bfs::Bfs;
use graph::dijkstra::Dijkstra;

#[test]
fn test_bfs_simple() {
	let mut g = Digraph::new(10);
	g.add_edge(0, 1);
	g.add_edge(0, 3);
	g.add_edge(0, 7);
	g.add_edge(1, 2);
	g.add_edge(2, 3);
	g.add_edge(2, 5);
	g.add_edge(3, 4);
	g.add_edge(4, 6);
	g.add_edge(5, 6);
	let mut bfs = Bfs::new(&g, 0, 6);
	bfs.find_path();
	// 0 -> 3 -> 4 -> 6
	assert_eq!(bfs.get_path().len(), 4);
	let path06 = vec![0, 3, 4, 6];
	assert_eq!(bfs.get_path(), &path06);
	bfs = Bfs::new(&g, 2, 6);
	bfs.find_path();
	// 2 -> 5 -> 6
	assert_eq!(bfs.get_path().len(), 3);
}

#[test]
fn test_dijkstra() {
	let mut g = Digraph::new(10);
	g.add_indirected_weighted_edge(0, 1, 4);
	g.add_indirected_weighted_edge(0, 7, 8);
	g.add_indirected_weighted_edge(1, 2, 8);
	g.add_indirected_weighted_edge(1, 7, 11);
	g.add_indirected_weighted_edge(2, 3, 7);
	g.add_indirected_weighted_edge(2, 8, 2);
	g.add_indirected_weighted_edge(2, 5, 4);
	g.add_indirected_weighted_edge(3, 4, 9);
	g.add_indirected_weighted_edge(3, 5, 14);
	g.add_indirected_weighted_edge(4, 5, 10);
	g.add_indirected_weighted_edge(5, 6, 2);
	g.add_indirected_weighted_edge(6, 7, 1);
	g.add_indirected_weighted_edge(6, 8, 6);
	g.add_indirected_weighted_edge(7, 8, 7);

	let mut dijkstra = Dijkstra::new(&g, 0, 3);
	dijkstra.find_path();
	// 0 -> 1 -> 2 -> 3
	//assert_eq!(dijkstra.get_path().len(), 4);
	let path03 = vec![0, 1, 2, 3];
	assert_eq!(dijkstra.get_path(), &path03);

	dijkstra = Dijkstra::new(&g, 0, 4);
	dijkstra.find_path();
	// 0 -> 7 -> 6 -> 5 -> 4
	let path04 = vec![0, 7, 6, 5, 4];
	assert_eq!(dijkstra.get_path(), &path04);
	
}