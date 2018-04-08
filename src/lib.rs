extern crate petgraph;
extern crate rayon;

pub mod trianglez {
    use petgraph::prelude::{
        UnGraph,
        NodeIndex
    };
    use petgraph::graphmap::NodeTrait;
    use petgraph::prelude::Graph;

    use rayon::prelude::*;

    use std::collections::HashMap;

    pub struct Triangle<N: NodeTrait> {
        nodes: Vec<N>
    }
    impl <N: NodeTrait> Triangle<N> {
        pub fn new(start: N, first: N, second: N) -> Triangle<N> {
            let mut v = vec![start, first, second];
            v.sort();
            Triangle {
                nodes: v
            }
        }
    }

    pub struct TriangleFinder<N: NodeTrait, E> {
        g: UnGraph<N, E>
    }

    impl <N: NodeTrait, E> TriangleFinder<N, E> {
        pub fn new() -> TriangleFinder<N, E> {
            TriangleFinder {
                g: Graph::new_undirected()
            }
        }

        fn find_triangles(&self) -> HashMap<N, Vec<Triangle<N>>> {
            self.g.node_indices().for_each(|start_ix| {
                let count_nbrs = self.g.neighbors(start_ix).count();
                (0..count_nbrs - 1).for_each(|first_ix| {
                    self.g.neighbors(start_ix).skip(first_ix).for_each(|second_ix| {
                        if self.g.contains_edge(NodeIndex::new(first_ix), second_ix) {
                            println!("Triangle: {:?}, {:?}, {:?}", start_ix, first_ix, second_ix)
                        }
                    })
                })
            });
            HashMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
