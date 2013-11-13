//! Graph library for Rust
//!
//! Provide an abstract Graph library and graph file output
//!
//!
//!

#[link(name = "rgraph",
       vers = "0.0.1",
       package_id = "rgraph",
       author = "letang.jeremy@gmail.com",
       uuid = "82D2FA35-9631-4CB7-8B25-201E83EB7CB3",
       url = "http://https://github.com/JeremyLetang/rgraph")];

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