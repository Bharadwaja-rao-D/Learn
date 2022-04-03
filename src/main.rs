use implimenting::graph::bfs;
use implimenting::graph::Graph;
use implimenting::graph::short_path::dijkistra;

fn main() {
    let graph = Graph::new_unweighted();
    let bf_tree = bfs::bfs(&graph);
    let shortest_path = dijkistra::dijkistra(&graph,0,4);

    println!("{:?}", bf_tree);
    println!("Shorest path: {:?}", shortest_path);
}
