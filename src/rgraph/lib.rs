//! Graph library for Rust
//!
//! Provide an abstract Graph library and graph file output
//!
//!
//!

#[crate_id = "rgraph#0.1"];
#[desc = "Graph library in Rust"];
#[license = "MIT"];
#[crate_type = "lib"];

#[deny(non_camel_case_types)];
#[deny(non_uppercase_statics)];
#[deny(unnecessary_qualification)];
// #[warn(missing_doc)];

extern mod extra;

// public reexports
pub use graph::Graph;
pub use graph::{Vertex, VertexIterator};
pub use graph::{Edge, EdgeIterator};

// mods
pub mod graph;
// pub mod graphviz;
// pub mod graphml;