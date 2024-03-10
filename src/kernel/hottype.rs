
use std::rc::Rc;

use super::HoTTerm;

// Term and Type are two graphs. And they refer to each other
// Type don't remember who is its term. But Term should remember its map
// We use term to construct expression: (in fact only application, in lambda calculus)
// Their correctness must be check by other function
#[derive(Clone)]
pub enum HoTType {
    // Anonymous type only has a name
    TyAnonymous(String),
    // Pair type has two argument A x B
    TyPair(Box<HoTType>, Box<HoTType>),
    // Sum type has two argument A + B
    TySum(Sum),
    // Sigma type has two argument, one is a base(it is a term) and the other is a type family
    TySigma(Box<Rc<HoTTerm>>, Box<HoTType>),
    // Identity is dependent on two terms. They must be in a same type
    TyIdentity(Box<Rc<HoTTerm>>, Box<Rc<HoTTerm>>),
    // Pi type has two argument
    TyPi(Box<HoTType>, Box<HoTType>),
    // function has its name, and we think the argument is an stack, the last is a target 
    TyFunc(String, Vec<Box<HoTType>>, Box<HoTType>),
    // the last must be a universe
    TyTypeFamily(String, Vec<Box<HoTType>>, Box<HoTType>),
    // similar as func
    TyLambda(Vec<Box<HoTType>>, Box<HoTType>),
    TyUniverse,
    // TODO: How to use a Natural?
    TyNat
}

#[derive(Clone)]
pub struct Sum {
    pub inl : Box<HoTType>,
    pub inr : Box<HoTType>
}
struct Sigma {
    base : String,

}

pub fn check_type(type1 : &HoTType, type2 : &HoTType) -> bool {

    match (type1, type2) {
        (HoTType::TyAnonymous(s0),
        HoTType::TyAnonymous(s1)) => {
            s0 == s1
        },
        (HoTType::TySum(p0), HoTType::TySum(p1)) => {
            check_type(p0.inl.as_ref(), p1.inl.as_ref()) &&
            check_type(p0.inr.as_ref(), p1.inr.as_ref())
        }
        (_,_) => false
    }
}



impl HoTType {
    pub fn mk_anonymous(name : &str) ->  HoTType {
        HoTType::TyAnonymous(name.to_string())
    }

    pub fn mk_sum(inl : &HoTType, inr : &HoTType) -> HoTType {
        HoTType::TySum(Sum { inl: Box::new(inl.clone()), inr: Box::new(inr.clone()) })
    }
}