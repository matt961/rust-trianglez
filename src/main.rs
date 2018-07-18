extern crate csv;
extern crate rayon;

mod graph;
mod readers;
mod trianglez;

use std::time::Instant;

use readers::read_fb_graph;
use std::fs;
use trianglez::TriangleFinder;
use rayon::prelude::*;

fn main() {
    let mut dir_reader = fs::read_dir("testres/").unwrap();
    let g = read_fb_graph(&mut dir_reader);
    println!("|V| = {} and |E| = {}", g.node_count(), g.edge_count());
    println!("Finding triangles and counting them all...");
    let time = Instant::now();
    let tf = TriangleFinder::find_triangles_par(&g);
    println!("Getting triangle count...");
    println!("found {} triangles.", tf.count());
    println!("Elapsed time {}s", time.elapsed().as_secs());
}
