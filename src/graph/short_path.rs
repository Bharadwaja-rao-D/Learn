pub mod dijkistra{
    use std::collections::BinaryHeap;

    use crate::graph::Graph;

    #[derive(Eq, PartialEq)]
    struct DijkistraNode{
        from: i32,
        to: i32,
        distance: i32,
    }

    impl Ord for DijkistraNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.distance.cmp(&other.distance).reverse()
        }
    }

    impl PartialOrd for DijkistraNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn dijkistra(graph: &Graph, src: i32, destination: i32) -> Vec<i32>{
        let mut path = vec![];
        let mut priority_queue: BinaryHeap<DijkistraNode> = BinaryHeap::new();
        priority_queue.push(DijkistraNode { from: src, to: src, distance: 0 });
        while !priority_queue.is_empty() {
            let src = priority_queue.pop().unwrap();
            path.push(value)
        }
        path

    }
}
