use graph::digraph::Digraph;
use graph::path_finder::PathFinder;
use graph::bfs::Bfs;

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
