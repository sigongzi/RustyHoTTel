use std::rc::Rc;

use super::{HoTType, HoTTerm};

#[derive(Clone, Copy)]
pub enum Tag {
    Sum(SumTag)
}

trait Inductive {
    fn inductive(&self, name_list : Vec<&str>) -> Vec<(Rc<HoTTerm>, Tag)>;
}


use super::hottype::Sum;

#[derive(Clone, Copy)]
pub enum SumTag {
    Inl,
    Inr
}

impl Inductive for Sum { 
    fn inductive(&self, name_list : Vec<&str>) -> Vec<(Rc<HoTTerm>, Tag)> {
        vec![
            (Rc::new(HoTTerm::new(name_list[0], self.inl.as_ref())), Tag::Sum(SumTag::Inl)),
            (Rc::new(HoTTerm::new(name_list[1], self.inr.as_ref())), Tag::Sum(SumTag::Inr))
        ]
    }
}

pub fn inductive_term(base : &HoTTerm, name_list : Vec<&str>) -> Vec<(Rc<HoTTerm>, Tag)> {
    let ty = base.get_type();
    match ty {
        HoTType::TySum(s) => s.inductive(name_list),
        _ => unreachable!("no inductive rules here")
    }
}

fn sum_comprehensive(tag_list : Vec<Tag>) -> bool {
    // maybe ordering does not matter, but...
    matches!(tag_list[0], Tag::Sum(SumTag::Inl)) &&
    matches!(tag_list[1], Tag::Sum(SumTag::Inr))
}

pub(crate) fn inductive_comprehensiveness(tag_list : Vec<Tag>) -> bool {
    // NOTE: we may check the length of tag_list?

    match tag_list[0] {
        Tag::Sum(_) => sum_comprehensive(tag_list),
        _ => false
    }
}