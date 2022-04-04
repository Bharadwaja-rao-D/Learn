use implimentation::graph::bfs;
use implimentation::graph::shortest_path;
use implimentation::graph::Graph;

fn main() {
    let graph = Graph::new_weighted();
    let bf_tree = bfs::bfs(&graph);

    println!("Breath first tree: {:?}", bf_tree);
    println!("Shortest path: {:?}" ,shortest_path::dijkstra(&graph, 0,1));
}
