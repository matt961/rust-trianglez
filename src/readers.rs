use csv::Reader;
use graph::Graph;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};

pub fn read_fb_graph<'a>(dir: &'a mut fs::ReadDir) -> Graph<String> {
    let mut g = Graph::new();
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

                            if id1 == id2 {
                                continue;
                            }

                            g.add_edge(id1, id2);
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
    use super::*;

    use std::fs;

    #[test]
    fn test_read_fb_graph() {
        let mut dir_reader = fs::read_dir("testres/").unwrap();
        let g = read_fb_graph(&mut dir_reader);
        println!("|V| = {} and |E| = {}", g.node_count(), g.edge_count());
    }
}
