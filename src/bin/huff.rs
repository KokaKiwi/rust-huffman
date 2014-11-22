extern crate graphviz;
extern crate huffman;

use std::io::{File, Command};
use huffman::{code, graph};

fn read_file(filename: &str) -> Vec<u8> {
    let mut f = File::open(&Path::new(filename));
    f.read_to_end().unwrap()
}

fn main() {
    let args = ::std::os::args();
    let ref filename = args[1];

    let data = read_file(filename.as_slice());
    let weights = code::calculate_weights(data.as_slice());
    let code = code::make_code(weights.as_slice()).expect("Failed to create code");

    let mut f = File::create(&Path::new("graph.dot"));
    graph::render(&mut f, "huffman", &code).unwrap();

    Command::new("dot").args(&["-Tsvg", "-O", "graph.dot"]).spawn().unwrap();
}
