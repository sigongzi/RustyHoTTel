use super::{hottype::{check_type, Sum}, HoTType};



fn construct_sum(type_parameter: Vec<&HoTType>, target_sum : &Sum) -> bool {
    type_parameter.len() == 1 &&
    (check_type(type_parameter[0], &target_sum.inl)
    || check_type(type_parameter[0], &target_sum.inr))
}

fn construct_pair(type_parameter: Vec<&HoTType>, target_a : &HoTType, target_b : &HoTType) -> bool {
    type_parameter.len() == 2 &&
    check_type(type_parameter[0], target_a) &&
    check_type(type_parameter[1], target_b)
}

pub fn is_able_to_construct_type(type_parameter: Vec<&HoTType>, target_type : &HoTType) -> bool {
    if type_parameter.iter().any(|&x| check_type(x, &HoTType::TyZero)) {
        return true;
    }
    match target_type {
        HoTType::TySum(s) => construct_sum(type_parameter, s),
        HoTType::TyPair(a, b) => construct_pair(type_parameter, a, b), // we can always construct pair from A and B
        HoTType::TyZero => false,
        HoTType::TyOne => true,
        _ => todo!("other constructor")
    }
}