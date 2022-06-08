pub mod bfs;
pub mod parsing;
pub mod shortest_paths;

#[derive(Debug, PartialEq)]
//The input of the graph will be in the format:
//v1, v2, v3 .....
//vi: (v1,w1), (v2,w2) ......
//for now:) vi: v1,w1; v2,w2 ......

pub struct Vertex {
    ///for now it is i32 but later will be a struct
    value: i32,
    pub location: (f64, f64),
}

impl Vertex {
    pub fn new(value: i32, (x, y): (f64, f64)) -> Self {
        return Vertex {
            value,
            location: (x, y),
        };
    }
    pub fn get(&self) -> i32 {
        return self.value;
    }
}

#[derive(Debug, PartialEq)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    //(adj_vertex, edge_weight)
    pub adj_list: Vec<Vec<(i32, i32)>>,
}
