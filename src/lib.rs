#![deny(unused_variables)]
mod tests;

pub mod linmap;
pub mod logmap;
pub mod map;

pub use crate::linmap::LinearMap;
pub use crate::logmap::LogMap;
