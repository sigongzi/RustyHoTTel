pub mod operator;
use std::rc::Rc;
#[derive(Clone)]
pub enum HoTType {
    TyAnonymous(String),
    TyPair(Pair)
}

#[derive(Clone)]
pub struct Pair {
    fst : Rc<HoTType>,
    snd : Rc<HoTType>
}

impl Pair {
    pub fn get_first(&self) -> Rc<HoTType> {
        self.fst.clone()
    }
    pub fn get_second(&self) -> Rc<HoTType> {
        self.snd.clone()
    }
}

// check the type recursively
pub fn check_type(type1 : Rc<HoTType>, type2 : Rc<HoTType>) -> bool {

    match (type1.as_ref(), type2.as_ref()) {
        (HoTType::TyAnonymous(s0),
        HoTType::TyAnonymous(s1)) => {
            s0 == s1
        },
        (HoTType::TyPair(p0), HoTType::TyPair(p1)) => {
            check_type(p0.fst.clone(), p1.fst.clone()) &&
            check_type(p0.snd.clone(), p1.snd.clone())
        }
        (_,_) => false
    }
}

pub mod constructor {
    use super::*;
    use std::rc::Rc;
    pub enum TyConstructor {
        Pair
    }
    fn construct_pair(parameter : Vec<Rc<HoTType>>) -> Result<Rc<HoTType>, String> {
        if parameter.len() != 2 {
            return Err("too many parameter".to_string());
        }
        Ok(
            Rc::new(
                HoTType::TyPair(Pair {
                    fst : Rc::clone(&parameter[0]),
                    snd : Rc::clone(&parameter[1])
                })
            )
        )
    }
    pub fn construct_type(type_kind : TyConstructor, parameter : Vec<Rc<HoTType>>) 
        -> Result<Rc<HoTType>, String> {
        match type_kind {
            TyConstructor::Pair => construct_pair(parameter),
        }        
    } 
}