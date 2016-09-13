#![recursion_limit="200"]

#[macro_use]
extern crate error_chain;

extern crate simplisp;

mod add_extensions;

mod error;

pub mod funcs;

pub use add_extensions::add_extensions;

pub use error::*;
