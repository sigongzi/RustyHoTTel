use std::rc::Rc;

use crate::kernel::HoTTerm;

/*use super::{HoTType, Pair};
pub enum Operator {
    Fst,
    Snd
}

fn operate_fst(op_kind : Operator, parameter : Vec<Rc<HoTTerm>>) 
    -> Result<Rc<HoTType>, String>{
    if parameter.len() != 1 {
        return Err("wrong number of term".to_string());
    }

    match parameter[0].get_type().as_ref() {
        HoTType::TyPair(p) => Ok(p.get_first()),
        _ => Err("wrong term type".to_string())
    }

}

fn operate_snd(op_kind : Operator, parameter : Vec<Rc<HoTTerm>>) 
    -> Result<Rc<HoTType>, String>{
    if parameter.len() != 1 {
        return Err("wrong number of term".to_string());
    }

    match parameter[0].get_type().as_ref() {
        HoTType::TyPair(p) => Ok(p.get_second()),
        _ => Err("wrong term type".to_string())
    }
}

pub fn apply_operator(op_kind : Operator, parameter : Vec<Rc<HoTTerm>>) 
    -> Result<Rc<HoTType>, String> {
    match op_kind {
        Operator::Fst => operate_fst(op_kind, parameter),
        Operator::Snd => operate_snd(op_kind, parameter)
    }
}*/