use crate::Graph;

///Remove the repeating edges
pub fn prims(graph: &Graph) -> Graph {
    let vertices = &graph.vertices;
    let mut mst = Graph {
        vertices: vertices.clone(),
        adj_list: Vec::with_capacity(graph.vertices.len()),
    };

    for index in 0..vertices.len() {}

    return mst;
}
