
use super::hottype::HoTType;

use std::rc::Rc;

/* A Term must remember its type

Note: It is not a compiler. But if it is a typechecker with a parser, it should compile a
expression and check 
*/

// TODO: A term should record where it comes from
// A term is unchangable after construction
pub struct HoTTerm {
    name : String,
    hottype: HoTType
}



impl HoTTerm {
    pub fn new(name : &str, hottype : &HoTType) -> HoTTerm {
        Self {
            name : name.to_string(),
            hottype : hottype.clone()
        }
    } 
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_type(&self) -> &HoTType {
        // just add a count to the smart pointer
        &self.hottype
    }
}

