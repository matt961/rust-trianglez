extern crate csv;
extern crate rayon;

mod graph;
mod readers;
mod trianglez;

use std::time::Instant;

use readers::read_fb_graph;
use std::fs;
use trianglez::{SeqTriangleFinder, ParTriangleFinder};
use rayon::prelude::*;

fn main() {
    let mut dir_reader = fs::read_dir("testres/").unwrap();
    let g = read_fb_graph(&mut dir_reader);
    println!("|V| = {} and |E| = {}", g.node_count(), g.edge_count());
    println!("Finding triangles and counting them all...");
    let time = Instant::now();
    let tf = SeqTriangleFinder::find_triangles(&g);
    println!("Getting triangle count...");
    println!("found {} triangles.", tf.count());
    let elapsed = time.elapsed();
    println!("Elapsed time {}s {}us", elapsed.as_secs(), elapsed.subsec_nanos());
    let time = Instant::now();
    println!("Doing Par now...");
    let tf_par = ParTriangleFinder::find_triangles(&g);
    let elapsed = time.elapsed();
    println!("Elapsed time {}s {}us", elapsed.as_secs(), elapsed.subsec_nanos());
    println!("found {} triangles.", tf_par.count());
}
