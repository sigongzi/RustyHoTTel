use super::{hottype::{check_type, Sum}, HoTType};



fn construct_sum(type_parameter: Vec<&HoTType>, target_sum : &Sum) -> bool {
    type_parameter.len() == 1 &&
    (check_type(type_parameter[0], &target_sum.inl)
    || check_type(type_parameter[0], &target_sum.inr))
}

pub fn is_able_to_construct_type(type_parameter: Vec<&HoTType>, target_type : &HoTType) -> bool {
    match target_type {
        HoTType::TySum(s) => construct_sum(type_parameter, s),
        _ => todo!("other constructor")
    }
}