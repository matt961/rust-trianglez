use std::vec::IntoIter;

use graph::{Graph, Node};

use std::fmt::Debug;

use rayon::prelude::ParallelIterator;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Triangle<N>(N, N, N);

pub struct TriangleFinder<'a, N: 'a> {
    local_triangles: Vec<Triangle<&'a N>>,
}

impl<'a, N> TriangleFinder<'a, N>
where
    N: Node + Debug,
{
    pub fn find_triangles(g: &Graph<N>) -> TriangleFinder<N> {
        let t = g.nodes()
            .flat_map(move |start| {
                g.neighbors(start)
                    .unwrap()
                    .enumerate()
                    .flat_map(move |(skip, first)| {
                        g.neighbors(start)
                            .unwrap()
                            .skip(skip + 1)
                            .filter(move |second| g.contains_edge(&first, &second))
                            .map(move |second| Triangle(start, first, second))
                    })
            })
            .collect();
        TriangleFinder { local_triangles: t }
    }

    pub fn find_triangles_par(g: &Graph<N>) -> TriangleFinder<N> {
        let t = g.nodes_par().into_par_iter().flat_map(move |start| {
            g.neighbors(&start)
                .unwrap()
                .filter(|first| g.degree(first) > g.degree(start))
                .enumerate()
                .flat_map(move |(skip, first)| {
                    g.neighbors(&start)
                        .unwrap()
                        .skip(skip + 1)
                        .filter(move |second| g.contains_edge(&first, &second))
                        .map(move |second| Triangle(start, first, second))
                }).collect::<Vec<_>>().into_par_iter()
        }).collect();
        TriangleFinder {
            local_triangles: t
        }
    }

    pub fn find_triangles_no_iters(g: &Graph<N>) -> TriangleFinder<N> {
        let mut t = Vec::with_capacity(g.edge_count());
        for start in g.nodes() {
            for (skip, first) in g.neighbors(start).unwrap().enumerate() {
                for second in g.neighbors(start).unwrap().skip(skip + 1) {
                    if g.contains_edge(&first, &second) {
                        t.push(Triangle(start, first, second));
                    }
                }
            }
        }
        TriangleFinder { local_triangles: t }
    }

    pub fn get_local_triangles(&self) -> impl Iterator<Item = &Triangle<&N>> {
        self.local_triangles.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut g: Graph<String> = Graph::new();
        g.add_edge("a".to_string(), "b".to_string());
        g.add_edge("b".to_string(), "c".to_string());
        g.add_edge("c".to_string(), "a".to_string());
        g.add_edge("d".to_string(), "a".to_string());
        g.add_edge("d".to_string(), "c".to_string());
        let tf = TriangleFinder::find_triangles(&g);
        for t in tf.get_local_triangles() {
            println!("{:?}", t);
        }
    }
}
