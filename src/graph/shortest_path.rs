use super::Graph;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Node{
    _value: i32,
    removed: bool,
    _x: i32,
    _y: i32,
}

impl Node {
    fn build_node(nodes: &Vec<super::Node>) -> Vec<Node>{
        nodes.into_iter().map(|ele| Node{_value: ele.value, _x: ele.x, _y: ele.y, removed: false}).collect()
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


//func is used to compute the heurestic value
fn astar<T: Fn(&super::Node) -> i32>(graph: &Graph, src: i32, destination: i32, func: &T) -> Vec<i32> {
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
                heap.push(PrioNode { from: src, to: *neighbors, weight: dist + weight + func(&graph.vertices[*neighbors as usize])});
            }
        }
        this_nodes[src as usize].removed = true;
    }


    let mut path = vec![];
    {
        let mut last = path_constructor.pop().unwrap();
        path_constructor.reverse();
        path.push(last.to);
        for item in path_constructor{
            if item.to == last.from {
               last = item;
               path.push(last.to);
            }
        }
    }

    path.reverse();
    path
}

pub fn dijkstra(graph: &Graph, src: i32, destination: i32)  -> Vec<i32>{
    let heuristic = |_ele: &super::Node| -> i32{
        0
    };
    astar(graph, src, destination, &heuristic)
}

pub fn astar_shortpath(graph: &Graph, src: i32, destination: i32)  -> Vec<i32>{
    let heuristic = |ele: &super::Node| -> i32{
        let destination_point = &graph.vertices[destination as usize];
        (((destination_point.x - ele.x).pow(2) + (destination_point.y - ele.y).pow(2)) as f64).sqrt() as i32
    };
    astar(graph, src, destination, &heuristic)
}

