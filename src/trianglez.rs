use petgraph::graphmap::NodeTrait;
use petgraph::prelude::UnGraphMap;

use std::collections::HashMap;
use std::collections::hash_map::Iter;

use std::fmt::Debug;

pub struct Triangle<N: NodeTrait>(N, N);

pub struct TriangleFinder<N: NodeTrait> {
    local_triangles: HashMap<N, Vec<Triangle<N>>>,
}

impl<N: NodeTrait + Debug> TriangleFinder<N> {
    pub fn find_triangles<E>(g: &UnGraphMap<N, E>) -> TriangleFinder<N> {
        let mut triangles = HashMap::new();
        for start in g.nodes() {
            triangles.insert(
                start,
                g.neighbors(start)
                    .enumerate()
                    .flat_map(|(skip, first)| {
                        g.neighbors(start)
                            .skip(skip + 1)
                            .filter(move |second| g.contains_edge(first, *second))
                            .map(move |second| Triangle(first, second))
                    })
                    .collect(),
            );
        }
        TriangleFinder {
            local_triangles: triangles,
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
