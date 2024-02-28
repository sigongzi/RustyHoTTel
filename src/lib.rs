pub mod proof;
pub mod kernel;
pub use proof::Proof;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::kernel::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test] 
    fn self_to_self() -> Result<(), String>{
        /* check p->p */
        let mut proof = Proof::new();

        
        proof.intro_atom_type("P")
            .intro_term("x","P")
            .target("P")
            .just("x")
            .map(|_| ())
    }

    #[test]
    fn commutity_of_and() -> Result<(), String> {
        /* check A x B -> B x A */
        let mut proof = Proof::new();
        proof.intro_atom_type("A")
        .intro_atom_type("B")
        .intro_atom_type("C")
        .construct_type(TyConstructor::Pair, vec!["A", "B"], "AxB")?
        .construct_type(TyConstructor::Pair, vec!["B", "A"], "BxA")?
        .target("BxA")
        .intro_term("x", "AxB")
        .operate_term(Operator::Fst, vec!["x"], "y")?
        .operate_term(Operator::Snd, vec!["x"], "z")?
        .construct_element("BxA", TermConstructor::Pair, vec!["z", "y"], "x0")?
        .just("x0")
        .map(|_| ())
    }

    #[test]
    fn assosiative_of_and() -> Result<(), String> {
        /* check A x (B x C) -> (A x B) x C */
        let mut proof = Proof::new();
        proof.intro_atom_type("A")
        .intro_atom_type("B")
        .intro_atom_type("C")
        .construct_type(TyConstructor::Pair, vec!["B", "C"], "BxC")?
        .construct_type(TyConstructor::Pair, vec!["A", "BxC"], "Ax(BxC)")?
        .construct_type(TyConstructor::Pair, vec!["A", "B"], "AxB")?
        .construct_type(TyConstructor::Pair, vec!["AxB", "C"], "(AxB)xC")?
        .target("(AxB)xC")
        .intro_term("x", "Ax(BxC)")
        .operate_term(Operator::Fst, vec!["x"], "y")?
        .operate_term(Operator::Snd, vec!["x"], "zz")?
        .operate_term(Operator::Fst, vec!["zz"], "z0")?
        .operate_term(Operator::Snd, vec!["zz"], "z1")?
        .construct_element("AxB", TermConstructor::Pair, vec!["y", "z0"], "x0")?
        .construct_element("(AxB)xC", TermConstructor::Pair, vec!["x0","z1"], "x1")?
        .just("x1")
        .map(|_| ())
    }

}
