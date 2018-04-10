use csv::Reader;
use petgraph::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};

pub fn read_fb_graph(dir: &mut fs::ReadDir) -> UnGraphMap<u64, ()> {
    let mut g = GraphMap::new();
    while let Some(Ok(dir_entry)) = dir.next() {
        let pb = dir_entry.path();
        let path = pb.as_path();
        if let Some(ext) = path.extension() {
            if let Some("csv") = ext.to_str() {
                let mut csv_reader = Reader::from_file(path).unwrap();
                let category = path.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split("_")
                    .nth(0)
                    .unwrap();
                for record in csv_reader.records() {
                    if let Ok(record) = record {
                        if let [id1, id2] = record.as_slice() {
                            let mut id1 = id1.clone();
                            id1.push_str(category);
                            let mut id2 = id2.clone();
                            id2.push_str(category);
                            // println!("id1 = {} and id2 = {}", id1, id2);

                            let mut h = DefaultHasher::new();

                            id1.hash(&mut h);
                            let node1 = h.finish();

                            h = DefaultHasher::new();
                            id2.hash(&mut h);
                            let node2 = h.finish();

                            if node1 == node2 {
                                continue;
                            }

                            g.add_edge(node1, node2, ());
                        }
                    }
                }
            }
        };
    }
    g
}

#[cfg(test)]
mod tests {
    use super::read_fb_graph;

    use std::fs;

    #[test]
    fn test_read_fb_graph() {
        let mut dir_reader = fs::read_dir("testres/just_artists/").unwrap();
        let g = read_fb_graph(&mut dir_reader);
        println!("|V| = {} and |E| = {}", g.node_count(), g.edge_count());
    }
}
