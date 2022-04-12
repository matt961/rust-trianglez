use graph::{Graph, Node};

use std::fmt::Debug;

use rayon::prelude::ParallelIterator;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Triangle<N>(N, N, N);

pub struct SeqTriangleFinder;

pub struct ParTriangleFinder;

impl SeqTriangleFinder {
    pub fn find_triangles<N: Node + Debug>(g: &Graph<N>) -> impl Iterator<Item = Triangle<&N>> {
        g.nodes().flat_map(move |start| {
            g.neighbors(start)
                .unwrap()
                .enumerate()
                .flat_map(move |(skip, first)| {
                    g.neighbors(start)
                        .unwrap()
                        .skip(skip + 1)
                        .filter_map(move |second| {
                            if g.contains_edge(first, second) {
                                Some(Triangle(start, first, second))
                            } else {
                                None
                            }
                        })
                })
        })
    }
}

impl ParTriangleFinder {
    pub fn find_triangles<N: Node + Debug>(
        g: &Graph<N>,
    ) -> impl ParallelIterator<Item = Triangle<&N>> {
        g.nodes_par().flat_map(move |start| {
            g.neighbors(start)
                .unwrap()
                .enumerate()
                .flat_map(move |(skip, first)| {
                    g.neighbors(&start)
                        .unwrap()
                        .skip(skip + 1)
                        .filter_map(move |second| {
                            if g.contains_edge(first, second) {
                                Some(Triangle(start, first, second))
                            } else {
                                None
                            }
                        })
                }).par_bridge()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_triangles_seq() {
        let mut g: Graph<String> = Graph::new();
        g.add_edge("a".to_string(), "b".to_string());
        g.add_edge("b".to_string(), "c".to_string());
        g.add_edge("c".to_string(), "a".to_string());
        g.add_edge("d".to_string(), "a".to_string());
        g.add_edge("d".to_string(), "c".to_string());
        let tf = SeqTriangleFinder::find_triangles(&g);
        for t in tf {
            println!("{:?}", t);
        }
    }

    #[test]
    fn it_finds_triangles_par() {
        let mut g: Graph<String> = Graph::new();
        g.add_edge("a".to_string(), "b".to_string());
        g.add_edge("b".to_string(), "c".to_string());
        g.add_edge("c".to_string(), "a".to_string());
        g.add_edge("d".to_string(), "a".to_string());
        g.add_edge("d".to_string(), "c".to_string());
        let tf = ParTriangleFinder::find_triangles(&g);
        for t in tf {
            println!("{:?}", t);
        }
    }
}
