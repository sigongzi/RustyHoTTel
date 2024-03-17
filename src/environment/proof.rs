use std::ops::Deref;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};

use crate::kernel::inductive::{inductive_comprehensiveness, Tag};
use crate::kernel::{self, hottype, is_able_to_construct_type, HoTTerm, HoTType, Operator};

use super::context::{self, Context};

pub type ContextRef = String;
pub type TermRef = String;
pub struct Proof {
    // context is a tree, but we need to save all its entity in the proof
    // all the modified on context is passed by proof
    context_set : HashMap<ContextRef, Context>,
    active_context : Option<ContextRef>,
    
    // In fact, we should see the proof as a context, because we need to construct some type dependent on term
    term_set : HashMap<String, Rc<HoTTerm>>,

    // other proof may use this list
    assumption_list : Vec<HoTType>,
    target : Option<HoTType>,
    is_end : bool

}
// some debug function 
impl Proof {
    fn target_type(&self) -> HoTType{
        self.target.as_ref().unwrap().clone()
    }
}

// Some helper function
impl Proof {
    // TODO: Should we remember the base element ?
    fn add_new_context(&mut self, base : Rc<HoTTerm>, new_term: Rc<HoTTerm>, inductive_tag : Tag,context_name : &str) {
        // we may need record some inductive type tag here

        let context = Context::new(new_term, self.active_context.clone(), inductive_tag);

        self.context_set.insert(context_name.to_string(), context);
    }
    fn context_add_new_term_wrapper(&mut self, context_name : Option<&ContextRef>, term_name : &str, ty: &HoTType) -> Result<&mut Proof, String> {
        let tm = HoTTerm::new(term_name, ty);
        match context_name {
            Some(s) => {
                self.context_set
                .get_mut(s)
                .ok_or(format!("no context with name {}", s))?
                .add_new_term(tm);
                
            },
            None => {
                self.term_set
                .insert(term_name.to_string(), Rc::new(tm));
            }
        }
        Ok(self)
    }

    // help find if the term is in the context
    fn search_term_recursively(&self, term_name : &str, context_name : Option<&ContextRef>) -> Result<Rc<HoTTerm>, &str> {
        match context_name {
            Some(s) => {
                let context = self.context_set
                .get(s)
                .unwrap();
                match context.is_term_exists(term_name) {
                    Some(term) => Ok(term),
                    None => self.search_term_recursively(term_name, context.get_parent_name())
                }

            },
            None => {
                self.term_set
                .get(term_name)
                .ok_or("there is no such term")
                .map(|a| a.clone())
            }
        }
    }

    fn search_term_helper(&self, term_name : &str) -> Result<Rc<HoTTerm>, &str>{
        match &self.active_context {
            Some(s) => {
                self.search_term_recursively(term_name, Some(&s))
            },
            None => {
                self.search_term_recursively(term_name, None)
            }
        }
    }

    fn check_context_contain(&self, context_list : Vec<&Context>, target_type : &HoTType) -> bool {
        context_list.iter().all(|&c|
        c.is_the_type_exists(target_type))
    }
}

impl Proof {
    pub fn new() -> Proof {
        Self { 
            context_set : HashMap::new(),
            active_context : None,
            term_set : HashMap::new(),
            assumption_list : Vec::new(),
            target : None,
            is_end : false
            
        }
    }

    pub fn assume(&mut self, name : &str, ty : &HoTType)  -> &mut Proof{
        // some temporary implement. Not we do not have too much assumption

        // The assumption list may be used one day. Proof can be the function for the other proof.
        self.assumption_list.push(ty.clone());

        // Save the term in the Proof Structure. Proof governs all the resources we need
        let term = HoTTerm::new(name, &ty);
        self.term_set.insert(name.to_string(), Rc::new(term));
        self
    }

    pub fn target(&mut self, ty: &HoTType) -> &mut Proof {
        // TODO: add a check here, if the list is end, then we cannot add new target
        
        // the last one in the list is a target
        self.assumption_list.push(ty.clone());
        self.target = Some(ty.clone());
        self.is_end = true;
        self
    }

    pub fn introduce(&mut self, term_name : &str) -> Result<&mut Proof, String> {
        let ty = self.target.as_ref().unwrap().clone();
        match ty {
            HoTType::TyFunc(mut func) => {
                // I must copy the name here... It is troubling
                let temp = self.active_context.clone();
                self.context_add_new_term_wrapper(temp.as_ref(), term_name, func.parameter.last().unwrap())?;
                // delete the first parameter
                func.parameter.pop();
                let l = func.parameter.len();
                if l == 0 {
                    self.target = Some(func.target.as_ref().clone());
                } else {
                    
                    self.target = Some(HoTType::TyFunc(func));
                }
                Ok(self)
            },
            _ => {Err("this target can not be introduced".to_string())}
        }
    }

    // construct new context for inductive type

    

    pub fn inductive(&mut self, term_name : &str, inductive_name : Vec<(&str,&str)>) -> Result<&mut Proof, String> {
        // we must use string here because... the &str belongs to proof?
        use crate::kernel::inductive::inductive_term;
        
        // TODO: need to check the option. The term may not be in the term_set
        // note that we may use the term in a context
        let t = self.search_term_helper(term_name)
        .map_err(|s| format!("Inductive: {}", s))?;
        let base = Rc::clone(&t);
        let res_term_tag = inductive_term(&base, inductive_name.iter().map(|s| s.1).collect());

        for i in 0..inductive_name.len() {
            self.add_new_context(
                Rc::clone(&base),
            Rc::clone(&res_term_tag[i].0), 
        res_term_tag[i].1,
    inductive_name[i].0);
        }

        Ok(self)
    }

    pub fn switch(&mut self, context_name : &str) -> &mut Proof {
        // TODO: here may add a check for the validity of the context name

        // it seems that we only need to change the tag of context
        self.active_context = Some(context_name.to_string());
        self
    }

    
    // Under here is some operation may fail. Where we need to check the type

    
    pub fn merge(&mut self, target_name : &str, context_name_list : Vec<&str>, target_type : &HoTType) -> Result<&mut Proof, String> {

        let context_list = context_name_list
        .iter()
        .map(|&s| self.context_set.get(s).ok_or("no such context")
        )
        .collect::<Result<Vec<&Context>, &str>>()?;
        
        let tag_list = context_list
        .iter()
        .map(|&c| c.get_tag())
        .collect::<Vec<Tag>>();

        let parent = context_list[0].get_parent_name()
        .map(|s| s.clone());
        // all the contexts must have this type
        if !self.check_context_contain(context_list, target_type) {
            return Err("not all the context contains such type".to_string());
        }

        if !inductive_comprehensiveness(tag_list) {
            return Err("this context can't be merged into its parent".to_string());
        }
        
        // NOTE: wait, we may check if the parent is the same one

        self.context_add_new_term_wrapper(parent.as_ref(), target_name, target_type)?;
        
        Ok(self)
    }

    pub fn just(&mut self, term_name : &str) -> Result<&mut Proof, String> {
        // all the term will be merged into the base set
        let term = self.term_set.get(term_name);
        match term {
            Some(t) => {
                let ty = t.get_type();
                println!("compare type {:?}, target type: {:?}", ty, self.target_type());
                if hottype::check_type(ty, self.target.as_ref().unwrap()) {
                    Ok(self)
                } else {
                    Err("types do not match with the target".to_string())
                }
                
            },
            None => {
            // I may know something. If your error message depends on the input, we have no choice except the string
                Err(format!("No term with name {}",term_name))
            }
        }
    }

    // SUM: A+B must point out the target type (Q: Is there any type inference technique for help?)
    pub fn construct(&mut self, target_type : &HoTType, parameter : Vec<&str>, name : &str) -> Result<&mut Proof, String> {

        // An annoying one, we need something lives longer than type_parameter
        let term_list = parameter.iter()
        .map(|&a| self.search_term_helper(a))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|s| format!("Construct: {}",s))?;

        let type_parameter = term_list.iter()
        .map(|a| a.get_type())
        .collect();

        if !is_able_to_construct_type(type_parameter, target_type) {
            return Err(format!("this parameter can not construct the type"));
        }
        let tm = HoTTerm::new(name, target_type);
        // add the term to the context
        match &self.active_context {
            Some(s) => {
                self.context_set
                .get_mut(s)
                .unwrap()
                .add_new_term(tm);
            }, 
            None => {
                self.term_set
                .insert(name.to_string(), Rc::new(tm));
            }
        }
        Ok(self)
    }

    pub fn operate(&mut self, op_kind : Operator, parameter : Vec<&str>, target_name : &str) ->
        Result<&mut Proof, String> {
        let term_list = parameter.iter()
        .map(|&s| 
            self.search_term_helper(s)
        )
        .collect::<Result<Vec<_>,_>>()
        .map_err(|s| format!("Operate: {}", s))?;
        let res_type = kernel::apply_operator(op_kind, term_list)?;

        // I must copy the name here... It is troubling
        let temp = self.active_context.clone();
        self.context_add_new_term_wrapper(temp.as_ref(), target_name, &res_type)?;

        Ok(self)
    }
}