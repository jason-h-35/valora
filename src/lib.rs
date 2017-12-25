#![feature(specialization)]
#![feature(use_nested_groups)]
#![feature(unboxed_closures)]
#![feature(fnbox)]
#![feature(nll)]
#![feature(crate_in_paths)]

#[macro_use]
extern crate error_chain;
#[macro_use]
pub extern crate glium;
extern crate lyon;
extern crate image;
pub extern crate rand;
extern crate petgraph;
extern crate itertools;
pub extern crate palette;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;

pub mod geom;
pub mod sketch;
pub mod patterns;
pub mod actors;
pub mod errors;
pub mod composition;

mod color;
mod properties;
mod tessellation;
mod gpu;
mod mesh;

pub use actors::*;
pub use errors::*;
pub use geom::*;
pub use patterns::*;
pub use sketch::*;
pub use color::*;
pub use mesh::*;
pub use composition::*;