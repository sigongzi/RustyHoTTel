
mod common;

use common::*;
use rustyhottel::{Environment, Proof};
use rustyhottel::kernel::HoTType;
#[test]
fn hello() {
    assert_eq!(3 + 4, 7);
}




// A idea to simplify them all: How about manipulate all the terms by their names?
#[test]
fn commutativity_of_or() -> Result<(), String>{
    // A + B -> B + A

    // let mut environment = Environment::new();
    // NOTE: It does not need environment now
    let mut proof = Proof::new();

    
    let base = type_helper(vec!["A", "B"]);
    let start_type = HoTType::mk_sum(&base[0], &base[1]);
    let target_type = HoTType::mk_sum(&base[1], &base[0]);
    proof.assume("x", &start_type)
    .target(&target_type)
    .inductive("x", vec![("inl","a"), ("inr", "b")])?
    .switch("inl")
    .construct(&target_type, vec!["a"], "b + a")?
    .switch("inr")
    .construct(&target_type, vec!["b"], "b + a")?
    // I can not find some better method except pointing out the target type explicitly
    .merge("y", vec!["inl","inr"], &target_type)?
    .just("y")
    .map(|_| ())
    .map_err(|s| s.to_string())
}   

#[test]
fn associativity_of_or() -> Result<(), String> {
    // (A + B) + C -> A + (B + C)

    // let mut environment = Environment::new();
    // NOTE: It does not need environment now
    let mut proof = Proof::new();
    let base = type_helper(vec!["A", "B", "C"]);

    let start_type = HoTType::mk_sum(&HoTType::mk_sum(&base[0], &base[1]), &base[2]);
    let target_type =HoTType::mk_sum(&base[0], &HoTType::mk_sum(&base[1], &base[2]));
    proof.assume("x", &start_type)
    .target(&target_type)
    .inductive("x", vec![("inl0","a+b"), ("inr0", "c")])?
    .switch("inl0")
    .inductive("a+b", vec![("inl00","a"),("inr00","b")])?
    .switch("inl00")
    .construct(&target_type, vec!["a"], "x0")?
    .switch("inr00")
    .construct(&HoTType::mk_sum(&base[1], &base[2]), vec!["b"], "x1")?
    .construct(&target_type, vec!["x1"], "x11")?
    .merge("y0", vec!["inl00", "inr00"], &target_type)?
    .switch("inr0")
    .construct(&HoTType::mk_sum(&base[1], &base[2]), vec!["c"], "x2")?
    .construct(&target_type, vec!["x2"], "y1")?
    .merge("z", vec!["inl0", "inr0"], &target_type)?
    .just("z")
    .map(|_| ())
    .map_err(|s| s.to_string())
    
}
