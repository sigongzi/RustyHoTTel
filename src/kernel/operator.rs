use std::rc::Rc;

use crate::kernel::{HoTTerm, HoTType};

#[derive(Clone, Copy)]
pub enum Operator {
    Fst,
    Snd
}



fn operate_fst(parameter : Vec<Rc<HoTTerm>>) 
    -> Result<HoTType, String>{
    if parameter.len() != 1 {
        return Err("wrong number of term".to_string());
    }

    match parameter[0].get_type() {
        HoTType::TyPair(a, _) => Ok(a.as_ref().clone()),
        _ => Err("wrong term type".to_string())
    }

}

fn operate_snd(parameter : Vec<Rc<HoTTerm>>) 
    -> Result<HoTType, String>{
    if parameter.len() != 1 {
        return Err("wrong number of term".to_string());
    }

    match parameter[0].get_type(){
        HoTType::TyPair(_, b) => Ok(b.as_ref().clone()),
        _ => Err("wrong term type".to_string())
    }
}

pub fn apply_operator(op_kind : Operator, parameter : Vec<Rc<HoTTerm>>) 
    -> Result<HoTType, String> {
    match op_kind {
        Operator::Fst => operate_fst(parameter),
        Operator::Snd => operate_snd(parameter)
    }
}