use implimentation::graph::bfs;
use implimentation::graph::shortest_path;
use implimentation::graph::Graph;

fn main() {
    let graph = Graph::new_weighted();
    let bf_tree = bfs::bfs(&graph);

    println!("Breath first tree: {:?}", bf_tree);
    println!("shortest path: {:?}" ,shortest_path::dijkstra(&graph, 0,5));
    println!("shortest path using heuristics: {:?}" ,shortest_path::astar_shortpath(&graph, 0,5));
}
