use petgraph::graphmap::NodeTrait;
use petgraph::prelude::UnGraphMap;

use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::time::SystemTime;

use std::fmt::Debug;

#[derive(Debug)]
pub struct Triangle<N: NodeTrait> {
    pub nodes: (N, N, N),
}

impl<N: NodeTrait> Triangle<N> {
    pub fn new(start: N, first: N, second: N) -> Triangle<N> {
        let mut v = &[start, first, second];
        v.sort();
        let (one, two, three) = v;
        Triangle {
            nodes: (one, two, three),
        }
    }
}

pub struct TriangleFinder<N: NodeTrait> {
    local_triangles: HashMap<N, Vec<Triangle<N>>>,
}

impl<N: NodeTrait + Debug> TriangleFinder<N> {
    pub fn find_triangles<E>(g: &UnGraphMap<N, E>) -> TriangleFinder<N> {
        TriangleFinder {
            local_triangles: g.nodes()
                .map(|start| {
                    (
                        start,
                        g.neighbors(start)
                            .enumerate()
                            .flat_map(|(skip, first)| {
                                g.neighbors(start)
                                    .skip(skip + 1)
                                    .filter(move |second| g.contains_edge(first, *second))
                                    .map(move |second| Triangle::new(start, first, second))
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    pub fn get_local_triangles(&self) -> Iter<N, Vec<Triangle<N>>> {
        self.local_triangles.iter()
    }
}

#[cfg(test)]
mod tests {
    use petgraph::prelude::*;
    use trianglez::TriangleFinder;

    #[test]
    fn it_works() {
        let mut g: UnGraphMap<&str, ()> = GraphMap::new();
        g.add_edge("a", "b", ());
        g.add_edge("b", "c", ());
        g.add_edge("c", "a", ());
        g.add_edge("d", "a", ());
        g.add_edge("d", "c", ());
        let tf = TriangleFinder::find_triangles(&g);
        for (key, triangles) in tf.get_local_triangles() {
            println!("For node {:?}...", key);
            for triangle in triangles {
                println!("{:?}", triangle);
            }
        }
    }
}
