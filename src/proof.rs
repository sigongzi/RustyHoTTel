

use std::collections::{HashSet, HashMap};
use std::rc::Rc;

use crate::kernel::{HoTTerm, HoTType, TyConstructor, Operator, self, TermConstructor, term};


pub struct Proof {
    // TODO: We may add some possible world property to the proof structure
    // all the element ownership is in the proof

    term_collection : HashMap<String, Rc<HoTTerm>>,
    // term collection: map the name of term to the concrete term
    // HoTTerm : name , Rc<Type>

    type_collection : HashMap<String, Rc<HoTType>>,
    proof_target : Option<Rc<HoTType>>,
}

impl Proof {
    pub fn new() -> Proof {
        Self { 
            term_collection : HashMap::new(), 
            type_collection : HashMap::new(),
            proof_target : None
        }
    }
    // add some anonymous type belongs to the universe 
    pub fn intro_atom_type(&mut self, name: &str) -> &mut Proof {
        self.type_collection.insert(name.to_string() ,
            Rc::new(HoTType::TyAnonymous(name.to_string())) 
        );
        self
    }

    pub fn intro_term(&mut self, name : &str, type_index : &str) -> &mut Proof {
        self.term_collection.insert(name.to_string(), 
            Rc::new(
                HoTTerm::new(name, 
                    Rc::clone(self.type_collection.get(type_index).unwrap()))
            )
        );
        self
    }

    pub fn construct_type(&mut self, construct_type : TyConstructor , 
        parameter : Vec<&str>, target_type_name : &str) -> Result<&mut Proof, String> {
        let ty_rc = kernel::construct_type(construct_type, 
            parameter
            .iter()
            .map(|&s| 
                Rc::clone(self.type_collection.get(s).unwrap())
            )
            .collect()
        )?;
        self.type_collection.insert(target_type_name.to_string(), ty_rc);
        Ok(self)
    }

    // use the operator to process the term 
    pub fn operate_term(&mut self, op_kind : Operator, parameter : Vec<&str>, target_name : &str) ->
        Result<&mut Proof, String> {
        let ty = kernel::apply_operator(
            op_kind, 
            parameter.iter()
            .map(
                |&s| 
                Rc::clone(self.term_collection.get(s).unwrap())
            )
            .collect()
        )?;
        self.term_collection.insert(target_name.to_string(), 
            Rc::new(
                HoTTerm::new(target_name, ty)
            ));
        Ok(self)
    }

    /* 
    it may be cumbersome because I can't find the type reference from a target type 
    If I know the target is A x B, I still can't find the concrete type reference

    there are two ways:
    the first one(taken now): pass the name
    the second one: construct the type again
    the third(?) one: there may be some method to find the type with same structure by some hash?
    (unique representation theorem?)
    */
    pub fn construct_element(&mut self, type_name : &str, term_constructor : TermConstructor, parameter : Vec<&str>, target_name : &str) -> Result<&mut Proof, String> {
        let ty = Rc::clone(self.type_collection.get(type_name).unwrap());

        let term = kernel::term_construct(
            term_constructor,
            parameter.iter()
            .map(|&s| 
                Rc::clone(self.term_collection.get(s).unwrap()))
            .collect(),
            target_name,
            ty
        )?;

        self.term_collection.insert(target_name.to_string(), term);
        Ok(self)
    }

    pub fn target(&mut self, type_index : &str) -> &mut Proof {
        self.proof_target = Some(
            Rc::clone(self.type_collection.get(type_index).unwrap())
        );
        self
    }

    pub fn just(&mut self, term_name : &str) -> Result<&mut Proof, String> {
        match self.term_collection.get(term_name) {
            None => {
                Err(format!("no term with name {}", term_name))
            },
            Some(term_rc) => {
                if kernel::check_type(
                    term_rc.get_type(),
                    self.proof_target.clone().unwrap()) {
                    Ok(self)
                }
                else {
                    Err(format!("type of term doesn't match the target"))
                }
            }
        }
    }
}