use implimentation::graph::bfs;
use implimentation::graph::Graph;

fn main() {
    let graph = Graph::new_unweighted();
    let bf_tree = bfs::bfs(&graph);

    println!("{:?}", bf_tree);
}
