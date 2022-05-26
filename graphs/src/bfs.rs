use crate::{Graph, Vertex};

impl Graph {
    //returns the vector of indices of vertices
    pub fn bfs(&self, source: usize) -> Vec<usize> {
        let mut bfs_traverse = vec![];
        let mut visited: Vec<usize> = vec![];
        let mut vertices: Vec<(&Vertex, bool)> =
            self.vertices
                .iter()
                .fold(vec![], |mut vertex_visted, vertex| {
                    vertex_visted.push((vertex, false));
                    return vertex_visted;
                });

        visited.push(source);
        vertices[source].1 = true;

        while !visited.is_empty() {
            let source = visited.remove(0);
            bfs_traverse.push(source);

            for (adj_vertex_index, _weight) in self.adj_list[source].iter() {
                let adj_vertex_index = *adj_vertex_index as usize;
                if !vertices[adj_vertex_index].1 {
                    visited.push(adj_vertex_index);
                    vertices[adj_vertex_index].1 = true;
                }
            }
        }

        return bfs_traverse;
    }
}

#[cfg(test)]
mod iterator {

    use crate::Graph;

    #[test]
    fn bfs_traverse() {
        let graph: Graph = include_str!("./input.txt").parse().unwrap();

        assert_eq!(vec![0, 1, 2], graph.bfs(0));
    }
}
