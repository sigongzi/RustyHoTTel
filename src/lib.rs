pub mod environment;
pub mod kernel;
pub use environment::{Proof, Environment};


pub fn add(left: usize, right: usize) -> usize {
    left + right
}


