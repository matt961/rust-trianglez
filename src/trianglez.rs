use graph::{Graph, Node};

use std::fmt::Debug;

use rayon::prelude::ParallelIterator;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Triangle<N>(N, N, N);

pub struct TriangleFinder;

impl TriangleFinder {
    pub fn find_triangles<N: Node+Debug>(g: &Graph<N>) -> impl Iterator<Item=Triangle<&N>> {
        g.nodes()
            .flat_map(move |start| {
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

    pub fn find_triangles_par<N: Node+Debug>(g: &Graph<N>) -> impl ParallelIterator<Item=Triangle<&N>> {
        g.nodes_par().flat_map(move |start| {
            g.neighbors(start)
                .unwrap()
                .enumerate()
                .flat_map(move |(skip, first)| {
                    g.neighbors(&start)
                        .unwrap()
                        .skip(skip + 1)
                        .filter(move |second| g.contains_edge(&first, &second))
                        .map(move |second| Triangle(start, first, second))
                }).collect::<Vec<_>>().into_par_iter()
        })
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
        for t in tf {
            println!("{:?}", t);
        }
    }
}
