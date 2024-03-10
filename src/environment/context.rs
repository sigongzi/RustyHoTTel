use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::kernel::hottype::check_type;
use crate::kernel::inductive::{self, Tag};
use crate::kernel::{hotterm, HoTTerm, HoTType};

use super::proof::ContextRef;

pub struct Context {
    // in fact their is only one term is newly build
    // A trivial implementation. But what is better ? 
    term_set : HashMap<String, Rc<HoTTerm>>,
    // we should find the term like a tree
    parent : Option<ContextRef>,
    // record what inductive branch it stays
    // TODO: We need build hash for term and hottype (It may be troubling)
    // type_set : HashSet<HoTType>,
    inductive_tag : Tag
}

impl Context {
    pub(crate) fn new(new_term : Rc<HoTTerm>, parent : Option<String>, inductive_tag : Tag) -> Self {
        //let type_set = HashSet::new();
        Self {
            term_set : vec![(new_term.get_name(), Rc::clone(&new_term))]
            .into_iter()
            .collect(),
            parent,
            inductive_tag
        }
    }

    pub(crate) fn add_new_term(&mut self, new_term : HoTTerm) {
        self.term_set.insert(new_term.get_name(), Rc::new(new_term));
    }

    pub(crate) fn is_term_exists(&self, term_name : &str) -> Option<Rc<HoTTerm>> {
        self.term_set
        .get(term_name)
        .map(|a| a.clone())
    }

    pub(crate) fn get_parent_name(&self) -> Option<&ContextRef> {
        self.parent.as_ref()
    }

    // TODO: Now I can only iterate all the term. But it is inefficient
    pub(crate) fn is_the_type_exists(&self, ty : &HoTType) -> bool {
        self.term_set
        .iter()
        .any(|(_, term)| check_type(term.get_type(), ty))
    }

    pub(crate) fn get_tag(&self) -> Tag{
        self.inductive_tag
    }
}