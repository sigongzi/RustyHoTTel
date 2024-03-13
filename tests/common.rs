use rustyhottel::kernel::HoTType;
pub fn type_helper(name : Vec<&str>) -> Vec<HoTType> {
    name.iter()
    .map(|&s| HoTType::TyAnonymous(
        s.to_string()
    ))
    .collect()
}