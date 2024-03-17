mod common;

use common::*;
use rustyhottel::{Environment, Proof};
use rustyhottel::kernel::{HoTType, Operator};
#[test]
fn hello() {
    assert_eq!(3 + 4, 7);
}

#[test]
fn self_for_self() -> Result<(), String> {
    // P -> P
    let mut proof = Proof::new();

    let base = type_helper(vec!["P"]);

    proof.assume("x", &base[0])
    .target(&base[0])
    .just("x")
    .map(|_| ())
    .map_err(|s| s.to_string())
}


fn build_transitive(env : &mut Environment, proof_name : &str) -> Result<(), String> {
    let mut proof = Proof::new();

    let base = type_helper(vec!["A", "B", "C"]);

    proof.assume("f", &HoTType::mk_func("AtoB", vec![&base[0]], &base[1]))
    .assume("g", &HoTType::mk_func("BtoC", vec![&base[1]],&base[2]))
    .target(&HoTType::mk_func("AtoC", vec![&base[0]], &base[2]))
    .introduce("x")?
    .operate(Operator::Apply, vec!["f", "x"], "y")?
    .operate(Operator::Apply, vec!["g", "y"], "z")?
    .just("z")
    .map(|_| ())
    .map_err(|s| s.to_string())
    // TODO : add the proof to the environment
}

#[test]
fn transitivity() -> Result<(), String> {
    // (A -> B) -> (B -> C) -> (A -> C)
    let mut env = Environment::new();
    build_transitive(&mut env, "transitive")
}

/*
it may need some type equation
#[test]
fn contrapositive() -> Result<(), String> {
    // (P -> Q) -> (Q -> 0) -> (P -> 0)
    Ok(())
}*/

#[test]
fn ex_falso() -> Result<(), String> {
    // 0 -> P
    let mut proof = Proof::new();
    let target_type = HoTType::mk_anonymous("P");
    proof.assume("a", &HoTType::TyZero)
    .target(&target_type)
    .construct(&target_type, vec!["a"], "x")?
    .just("x")
    .map(|_|())
    .map_err(|s| s.to_string())
}

#[test]
fn one_to_p_imply_p() -> Result<(), String> {
    // (1 -> P) -> P
    let mut proof = Proof::new();
    let base = type_helper(vec!["P"]);
    let start = HoTType::mk_func("f", vec![&HoTType::TyOne], &base[0]);
    proof.assume("f", &start)
    .target(&base[0])
    .construct(&HoTType::TyOne, vec![], "unit")?
    .operate(Operator::Apply, vec!["f","unit"], "x")?
    .just("x")
    .map(|_|())
    .map_err(|s| s.to_string())
}
