use std::rc::Rc;

use crate::kernel::{hottype::check_type, HoTTerm, HoTType};

#[derive(Clone, Copy)]
pub enum Operator {
    Fst,
    Snd,
    Apply
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

fn do_apply(parameter : Vec<Rc<HoTTerm>>) -> Result<HoTType, String>{
    let f = &parameter[0];
    match f.get_type() {
        HoTType::TyFunc(_) => (),
        _ => {return Err("can not apply using something not funclike".to_string());}
    };
    if let HoTType::TyFunc(mut func) = f.get_type().clone() {
        for i in 1..parameter.len() {
            if !check_type(func.parameter.last().unwrap(), parameter[i].get_type()) {
                return 
                Err(format!("error in apply parameter {}, type does not match", parameter[i].get_name()));
            }
            func.parameter.pop();
        }
        let ty = match func.parameter.len() {
            0 => func.target.as_ref().clone(),
            _ => HoTType::TyFunc(func)
        };
        return Ok(ty);
    }
    unreachable!();
}

pub fn apply_operator(op_kind : Operator, parameter : Vec<Rc<HoTTerm>>) 
    -> Result<HoTType, String> {
    match op_kind {
        Operator::Fst => operate_fst(parameter),
        Operator::Snd => operate_snd(parameter),
        Operator::Apply => do_apply(parameter)
    }
}