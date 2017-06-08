mod pattern_parser;

pub fn read_dir(path: &str) {
    use std::fs;

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
    }


} 