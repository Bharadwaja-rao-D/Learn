use std::{num::ParseIntError, str::FromStr};

use crate::{Graph, Vertex};

// TODO : Complete this macro to convert (1,2), (3,4) to 1,2; 3,4
// #[macro_export]
// macro_rules! parenth_semicolon_form {
//     ($vertex: expr, ) => {};
// }

impl FromStr for Vertex {
    type Err = ParseIntError;

    //need to add location later
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Vertex::new(s.trim().parse()?, (0.0, 0.0)));
    }
}

impl FromStr for Graph {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (vertices, adj_stuff) = s.split_once('\n').unwrap();

        let vertices: Vec<Vertex> = vertices
            .split(',')
            .map(|vertex| vertex.parse().unwrap())
            .collect();

        let no_vertices = vertices.len();

        let mut adj_list: Vec<Vec<(i32, i32)>> = Vec::with_capacity(no_vertices);
        for _ in 0..no_vertices {
            adj_list.push(vec![]);
        }

        for line in adj_stuff.lines() {
            let (curr_ver, adj_vertices) = line.split_once(':').unwrap();
            let curr_ver: usize = curr_ver.parse().unwrap();
            adj_list[curr_ver] =
                adj_vertices
                    .split(';')
                    .fold(vec![], |mut whole_vec, particular| {
                        let (adj, weight) = particular.split_once(',').unwrap();
                        let adj = adj.trim().parse().unwrap();
                        let weight = weight.parse().unwrap();
                        whole_vec.push((adj, weight));
                        return whole_vec;
                    });
        }
        return Ok(Graph { vertices, adj_list });
    }
}

#[cfg(test)]
mod str_to_graph {

    use super::Graph;
    use super::Vertex;

    #[test]
    fn vertex_parse() {
        assert_eq!(Vertex::new(1, (0.0, 0.0)), "1".parse().unwrap());
    }

    #[test]
    fn graph_construction() {
        let input = include_str!("./input.txt");
        let expected_graph = Graph {
            vertices: vec![
                Vertex::new(1, (0.0, 0.0)),
                Vertex::new(2, (0.0, 0.0)),
                Vertex::new(3, (0.0, 0.0)),
            ],
            adj_list: vec![vec![(1, 1), (2, 2)], vec![(0, 0)], vec![(0, 2)]],
        };

        assert_eq!(expected_graph, input.parse().unwrap())
    }
}
