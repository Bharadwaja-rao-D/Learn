pub mod bfs;
pub mod shortest_path;

///Node is just an integer here
///adjacency_list[i] contains the index of nodes adjacent to the node vertices[i]

pub struct Node{
    value: i32,
}

impl Node{
    fn new(value: i32) -> Node {
        Node { value }
    }
}

pub struct Graph{
    vertices: Vec<Node>,
    ///node index and the weight
    ///vec[i] contains the indices of its adjacent nodes which have a to path from i
    adjacency_list: Vec<Vec<(i32, i32)>>,
}

impl Graph{
    ///actually does some complex operations
    ///sets Seen value of all vertices to not_yet_touched
    fn add_unweights(neighbors: Vec<i32>) -> Vec<(i32, i32)> {
        neighbors.into_iter().map(|index| (index,0)).collect()
    }

    pub fn new_unweighted() -> Graph{
        Graph { vertices: vec![Node::new(0),
        Node::new(1),
        Node::new(2),
        Node::new(3),
        Node::new(4),
        Node::new(5),
        Node::new(32)],
        adjacency_list: vec![ Graph::add_unweights(vec![1,4]), Graph::add_unweights(vec![0,2,5]),
                        Graph::add_unweights(vec![1,4]), Graph::add_unweights(vec![4]), Graph::add_unweights(vec![0,2,3]),
                        Graph::add_unweights(vec![1,6]), Graph::add_unweights(vec![5]) ] }

    }

    pub fn new_weighted() -> Graph {
        Graph { vertices: vec![ Node::new(0),
        Node::new(1),
        Node::new(2),
        Node::new(3),
        Node::new(4),
        Node::new(5),
        ],
        adjacency_list: vec![
            vec![(1,7), (2,2), (4,3)],
            vec![(2,3)],
            vec![(0,2), (1,3), (3,1)],
            vec![(2,1), (5,2)],
            vec![(0,3), (5,3)],
            vec![(3,2), (4,3)],
        ] }
    }
}

