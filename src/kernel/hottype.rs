
use std::rc::Rc;

use super::HoTTerm;

// Term and Type are two graphs. And they refer to each other
// Type don't remember who is its term. But Term should remember its map
// We use term to construct expression: (in fact only application, in lambda calculus)
// Their correctness must be check by other function
#[derive(Clone, Debug)]
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
    TyFunc(Func),
    // the last must be a universe
    TyTypeFamily(String, Vec<Box<HoTType>>, Box<HoTType>),
    // similar as func
    TyLambda(Vec<Box<HoTType>>, Box<HoTType>),
    TyZero,
    TyOne,
    TyUniverse,
    // TODO: How to use a Natural?
    TyNat
}

#[derive(Clone, Debug)]
pub struct Sum {
    pub inl : Box<HoTType>,
    pub inr : Box<HoTType>
}

#[derive(Clone, Debug)] 
pub struct Func {
    pub name : String,
    pub parameter : Vec<Box<HoTType>>, // it is a stack
    pub target : Box<HoTType>
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
        },
        (HoTType::TyPair(pa, pb), 
        HoTType::TyPair(qa, qb)) => {
            check_type(&pa, &qa) &&
            check_type(&pb, &qb)
        },
        (HoTType::TyFunc(fa), HoTType::TyFunc(fb)) => {
            fa.parameter.len() == fb.parameter.len() &&
            fa.parameter.iter().zip(fb.parameter.iter()).all(|(a,b)|
            check_type(a.as_ref(), b.as_ref())) &&
            check_type(&fa.target, &fb.target)
        },
        (HoTType::TyZero, HoTType::TyZero) => true,
        (HoTType::TyOne, HoTType::TyOne) => true,
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

    pub fn mk_pair(left : &HoTType, right : &HoTType) -> HoTType {
        HoTType::TyPair(Box::new(left.clone()), Box::new(right.clone()))
    }
    pub fn mk_func(name : &str, func_parameter : Vec<&HoTType>, target : &HoTType) -> HoTType {
        HoTType::TyFunc(Func {
            name : name.to_string(),
            // add parameter in the reversed order
            parameter : func_parameter.iter().rev().map(|&s| Box::new(s.clone())).collect(),
            target : Box::new(target.clone())
        })
    }
}