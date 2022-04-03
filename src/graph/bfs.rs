use super::*;

#[derive(PartialEq)]
pub enum Seen {
    Discovered,
    InQueue,
    NotYetTouched
}

struct BfNode{
    value: i32,
    discovery_status: Seen,
    layer: u8
}

impl BfNode {
   fn new(value: i32) -> BfNode {
       BfNode{value, discovery_status:Seen::NotYetTouched, layer: 0}
    }

   fn construct_bf_nodes(vertices: &Vec<Node>) -> Vec<BfNode>{
       let mut bf_vertices = vec![];

       for vertex in vertices {
           bf_vertices.push(BfNode::new(vertex.value));
       }

       bf_vertices
   }
}

///returns vector of list of vertices in a layer
///we deal this with only un weighted graph here
pub fn bfs(graph: &Graph) -> Vec<(i32,u8)>{
    let mut layers: Vec<(i32,u8)> = vec![];
    let mut queue: Vec<i32> = vec![];


    let mut vertices = BfNode::construct_bf_nodes(&graph.vertices);

    queue.push(0);
    vertices[0].discovery_status = Seen::InQueue;

    while !queue.is_empty() {
        let present_vertex_index = queue.remove(0);
        vertices[present_vertex_index as usize].discovery_status = Seen::Discovered;

        //add all adjacent vertices into the queue

        for (adjacent,_) in &graph.adjacency_list[present_vertex_index as usize]{
            if vertices[*adjacent as usize].discovery_status == Seen::NotYetTouched {
                queue.push(*adjacent);
                vertices[*adjacent as usize].discovery_status = Seen::InQueue;
                let count = vertices[present_vertex_index as usize].layer + 1;
                vertices[*adjacent as usize].layer =  count;
                layers.push((vertices[*adjacent as usize].value, count));
            }
        }
    }
    layers
}
