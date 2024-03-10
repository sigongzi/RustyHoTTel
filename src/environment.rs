pub mod context;
pub mod proof;

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

pub use proof::Proof;
pub type ProofRef = Rc<RefCell<Proof>>;

// 2024.3.9 : only consider the empty environment here
pub struct Environment {
    // the ownership of proof is in the environment
    proof_set : HashMap<String, ProofRef>,

    // type_family :
    // operation : 
}

impl Environment {
    pub fn new() -> Environment {
        Self {
            proof_set : HashMap::new() 
        }
    }

}