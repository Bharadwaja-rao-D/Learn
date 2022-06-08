use crate::{Graph, Vertex};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrioInfo {
    pub to: usize,
    pub from: usize,
    pub distance: usize,
}

impl Ord for PrioInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.distance.cmp(&other.distance).reverse();
    }
}

impl PartialOrd for PrioInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

pub fn dijkstra(graph: &Graph, from: usize, to: usize) -> Vec<usize> {
    let mut prio_que = std::collections::BinaryHeap::new();
    let mut path = Vec::new();
    let mut output_path = vec![];
    prio_que.push(PrioInfo {
        to: from,
        from,
        distance: 0,
    });
    let vertices = &graph.vertices;
    let mut vertices: Vec<(&Vertex, bool)> =
        vertices.iter().map(|vertex| (vertex, false)).collect();
    let adj = &graph.adj_list;
    while !prio_que.is_empty() {
        let src = prio_que.pop().unwrap();
        path.push(src.clone());
        let src = src.to;

        if src == to {
            //backtracing

            let mut destination = to;
            while !path.is_empty() {
                let this_node = path.pop().unwrap();
                if this_node.to == destination {
                    output_path.push(this_node.to);
                    destination = this_node.from;
                }
            }

            output_path.reverse();
            return output_path;
        }

        //add all adjacent vertices of the src to the prio_que
        for (adj_vertex, weight) in &adj[src] {
            if !vertices[*adj_vertex as usize].1 {
                prio_que.push(PrioInfo {
                    to: *adj_vertex as usize,
                    from: src,
                    distance: *weight as usize,
                })
            }
        }

        vertices[src].1 = true;
    }

    return output_path;
}

#[cfg(test)]
mod shortest_paths {
    use crate::Graph;

    use super::dijkstra;

    #[test]
    fn dijkstra_test() {
        println!("Here");
        let input = include_str!("./input2.txt");
        let graph: Graph = input.parse().unwrap();

        assert_eq!(dijkstra(&graph, 0, 5), vec![0, 2, 3, 5]);
    }
}
