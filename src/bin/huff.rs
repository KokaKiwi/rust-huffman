extern crate graphviz;
extern crate huffman;

use std::hash::Hash;
use std::fmt::Show;
use std::io::{File, Command};
use huffman::{code, graph};

fn read_file(filename: &str) -> Vec<u8> {
    let mut f = File::open(&Path::new(filename));
    f.read_to_end().unwrap()
}

#[allow(dead_code)]
fn write_code<T: Ord + Copy + Hash + Show>(filename: &str, code: &code::Node<T>) {
    let mut f = File::create(&Path::new(filename));
    graph::render(&mut f, filename, code).unwrap();

    Command::new("dot").args(&["-Tsvg", "-O", filename]).spawn().unwrap();
}

fn main() {
    let args = ::std::os::args();
    let ref filename = args[1];

    let data = read_file(filename.as_slice());
    let weights = code::calculate_weights(data.as_slice());
    let code = code::make_code(weights.as_slice()).expect("Failed to create code");
    let table = code.table();

    // write_code("graph.dot", &code);
}
