use super::Graph;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Node{
    _value: i32,
    removed: bool,
}

impl Node {
    fn build_node(nodes: &Vec<super::Node>) -> Vec<Node>{
        nodes.into_iter().map(|ele| Node{_value: ele.value, removed: false}).collect()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct PrioNode{
    from: i32,
    to: i32,
    weight: i32,
}

impl Ord for PrioNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

impl PartialOrd for PrioNode{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


pub fn dijkstra(graph: &Graph, src: i32, destination: i32) -> Vec<i32> {
    let mut path_constructor = vec![];
    let mut heap: BinaryHeap<PrioNode> = BinaryHeap::new();
    let mut this_nodes = Node::build_node(&graph.vertices);

    heap.push(PrioNode { from: src, to: src, weight: 0 });

    while !heap.is_empty() {
        //here u r sure that u will reach destination before the heap becomes empty
        let min = heap.pop().unwrap();
        let src = min.to;
        let dist = min.weight;

        path_constructor.push(min);

        if src == destination {
            break;
        }

        for (neighbors, weight) in &graph.adjacency_list[src as usize]{
            if !this_nodes[*neighbors as usize].removed {
                heap.push(PrioNode { from: src, to: *neighbors, weight: dist + weight });
            }
        }
        this_nodes[src as usize].removed = true;
    }

    println!("path_constructor: {:?}", path_constructor);

    path_constructor.iter().fold(vec![], |mut path, ele| {
        path.push(ele.from);
        path
    })
}
