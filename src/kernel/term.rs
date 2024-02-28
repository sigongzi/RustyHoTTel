
use super::hottype::HoTType;

use std::rc::Rc;
pub struct HoTTerm {
    name : String,
    hottype: Rc<HoTType>
}



impl HoTTerm {
    pub fn new(name : &str, hottype : Rc<HoTType> ) -> HoTTerm {
        Self {
            name : name.to_string(),
            hottype
        }
    } 
    pub fn get_type(&self) -> Rc<HoTType> {
        self.hottype.clone()
    }
}

pub mod constructor {
    use std::rc::Rc;

    use crate::kernel::HoTType;

    use super::HoTTerm;

    pub enum TermConstructor {
        Pair
    }

    fn construct_pair(parameter : Vec<Rc<HoTTerm>>, name : &str, ty : Rc<HoTType>) -> 
    Result<Rc<HoTTerm>, String>{
        if parameter.len() != 2 {
            return Err("wrong number of parameter".to_string());
        }

        Ok(
            Rc::new(
                HoTTerm::new(name, ty)
            )
        )
    }
    pub fn term_construct(term_const : TermConstructor, parameter : Vec<Rc<HoTTerm>>,  name : &str, ty : Rc<HoTType>) -> Result<Rc<HoTTerm>, String>{
        match term_const {
            TermConstructor::Pair => construct_pair(parameter, name, ty)
        }
    }
}