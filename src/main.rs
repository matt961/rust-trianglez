#![feature(duration_extras)]
extern crate csv;
extern crate petgraph;
extern crate rayon;

mod trianglez;
mod readers;

use readers::read_fb_graph;
use std::fs;
use petgraph::graphmap::UnGraphMap;
use trianglez::TriangleFinder;

fn main() {
    bench_big_graph();
}

fn bench_big_graph() {
    let mut dir_reader = fs::read_dir("testres/").unwrap();
    let g = read_fb_graph(&mut dir_reader);
    println!("|V| = {} and |E| = {}", g.node_count(), g.edge_count());
    println!("Finding triangles and counting them all...");
    let tf = TriangleFinder::find_triangles(&g);
    let count: usize = tf.get_local_triangles().map(|t_arr| t_arr.1.len()).sum();
    println!("found {} triangles.", count);
    //println!("Number of triangles = {}", count);
}