#![experimental]
#![feature(phase, default_type_params)]

#[phase(plugin, link)]
extern crate log;
extern crate graphviz;

pub mod code;
pub mod graph;
pub mod table;
mod bits;
mod ord;
