mod common;

use common::*;
use rustyhottel::{Environment, Proof};
use rustyhottel::kernel::{HoTType, Operator};
#[test]
fn hello() {
    assert_eq!(3 + 4, 7);
}




// A idea to simplify them all: How about manipulate all the terms by their names?
#[test]
fn commutativity_of_and() -> Result<(), String>{
    // A x B -> B x A

    // let mut environment = Environment::new();
    // NOTE: It does not need environment now
    let mut proof = Proof::new();

    
    let base = type_helper(vec!["A", "B"]);
    let start_type = HoTType::mk_pair(&base[0], &base[1]);
    let target_type = HoTType::mk_pair(&base[1], &base[0]);
    println!("{:?}", target_type);
    proof.assume("x", &start_type)
    .target(&target_type)
    .operate(Operator::Fst, vec!["x"], "x0")?
    .operate(Operator::Snd, vec!["x"], "x1")?
    .construct(&target_type, vec!["x1","x0"], "y")?
    .just("y")
    .map(|_| ())
    .map_err(|s| s.to_string())

}   

#[test]
fn associativity_of_or() -> Result<(), String> {
    // (A x B) x C -> A x (B x C)

    // let mut environment = Environment::new();
    // NOTE: It does not need environment now
    let mut proof = Proof::new();

    
    let base = type_helper(vec!["A", "B", "C"]);
    let left = HoTType::mk_pair(&base[0], &base[1]);
    let right = HoTType::mk_pair(&base[1], &base[2]);
    let start = HoTType::mk_pair(&left, &base[2]);
    let target = HoTType::mk_pair(&base[0], &right);
    proof.assume("x", &start)
    .target(&target)
    .operate(Operator::Fst, vec!["x"], "y")?
    .operate(Operator::Snd, vec!["x"], "x1")?
    .operate(Operator::Fst, vec!["y"], "y0")?
    .operate(Operator::Snd, vec!["y"], "y1")?
    .construct(&right, vec!["y1","x1"], "bc")?
    .construct(&target, vec!["y0", "bc"], "abc")?
    .just("abc")
    .map(|_| ())
    .map_err(|s| s.to_string())
    
}
