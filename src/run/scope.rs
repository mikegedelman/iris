use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::Value;
use super::typing::*;


#[derive(Clone, Debug)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    context: String,
    vars: HashMap<String, Value>,
    methods: HashMap<(String, TypeDefinition), Function>,
}

impl Scope {
    pub fn new(context: String) -> Scope {
        Scope{ context, vars: HashMap::new(), methods: HashMap::new(), parent: None, }
    }

    /// Associated fn instead of a method because we want to clone an Rc
    /// to an existing RefCell, so we accept that instead
    pub fn nest(parent: Rc<RefCell<Scope>>, context: &str) -> Scope  {
        Scope {
            parent: Some(Rc::clone(&parent)),
            context: context.to_string(),
            vars: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn declare_var(&mut self, name: &str, val: Value) {
        assert!(!self.vars.contains_key(name), format!("can't redeclare var {}", name));
        self.vars.insert(name.to_string(), val);
    }

    pub fn set_var(&mut self, name: &str, val: Value) {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), val);
        } else {
            match &self.parent {
                Some(p) => {
                    let mut parent = p.borrow_mut();
                    parent.set_var(name, val);
                },
                None => panic!("can't assign to undeclared var \"{}\" | context: {}", name, self.context),
            }
        }
    }

    pub fn get_var(&self, name: &str) -> Option<Value> {
        match self.vars.get(name) {
            Some(x) => Some(x.clone()),
            None => match &self.parent {
                        Some(p) => {
                            let parent = p.borrow();
                            parent.get_var(name)
                        },
                        None => None,
                    }
        }
    }

    pub fn var_is_set(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    pub fn declare_method(&mut self, name: &str, for_type: TypeDefinition, func: Function) {
        self.methods.insert((name.to_string(), for_type), func);
    }

    pub fn get_method(&self, name: &str, typ: TypeDefinition) -> Option<Function> {
        let search = self.methods.get(&(name.to_string(), typ.clone()));
        match search {
            Some(f) => Some(f.clone()),
            None => match &self.parent {
                Some(p) => {
                    let parent = p.borrow();
                    parent.get_method(name, typ)
                },
                None => None,
            },
        }
    }

    pub fn get_fn(&self, name: &str) -> Function {
        let val = self.get_var(name).expect(&format!("unknown function {}", name));
        match val {
            Value::Function(f) => f,
            _ => panic!("{:?} is not a function", val)
        }
    }
    // If first arg, check to see if name & arg type combo is registered as an interface
    // Otherwise check the scope vars for the name and verify it's a function
}