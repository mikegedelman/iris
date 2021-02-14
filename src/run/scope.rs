use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::Value;
use super::typing::*;

/// All program state is stored in a Scope.
#[derive(Clone, Debug)]
pub struct Scope {
    /// A scope may have a parent, which is stored in a reference-counted RefCell.
    /// If it doesn't have a parent, it is the top-level global scope.
    /// This allows support for closures: multiple closures can hold references to
    /// the parent scope, and mutate it when called.
    /// RefCell is needed to provide interior mutability, because Rust normally
    /// doesn't like you holding multiple mutable references like this.
    parent: Option<Rc<RefCell<Scope>>>,

    /// The context can be set to things like the function name, for better error messages
    context: String,

    /// Lookups for this scope's variables and methods
    vars: HashMap<String, Value>,
    methods: HashMap<(String, TypeDefinition), Function>,
}

impl Scope {
    pub fn new(context: String) -> Scope {
        Scope{ context, vars: HashMap::new(), methods: HashMap::new(), parent: None, }
    }

    /// Create a new Scope as a child of the given Scope.
    /// This is an associated fn instead of a method because we want to clone an Rc
    /// to an existing RefCell, so we accept that instead
    pub fn nest(parent: &Rc<RefCell<Scope>>, context: &str) -> Scope  {
        Scope {
            parent: Some(Rc::clone(parent)),
            context: context.to_string(),
            vars: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    /// Declare a variable in the current scope, unless it has already been declared
    pub fn declare_var(&mut self, name: &str, val: Value) {
        assert!(!self.vars.contains_key(name), format!("can't redeclare var {}", name));
        self.vars.insert(name.to_string(), val);
    }

    /// Assign a new value to an existing variable, which may exist in this Scope or
    /// one of its parents
    pub fn set_var(&mut self, name: &str, val: Value) {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), val);
        } else {
            // Walk up the hierarchy to check if the variable exists in a parent scope
            // This pattern is repeated a few times in other methods
            match &self.parent {
                Some(p) => {
                    let mut parent = p.borrow_mut();
                    parent.set_var(name, val);
                },
                None => panic!("can't assign to undeclared var \"{}\" | context: {}", name, self.context),
            }
        }
    }

    /// Maybe read a variable from the current scope or its parents
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

    /// Quickly check if a variable is set in the current scope
    /// TODO: this should probably check parent scopes too?
    pub fn var_is_set(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    /// Declare a method in the current scope
    pub fn declare_method(&mut self, name: &str, for_type: TypeDefinition, func: Function) {
        self.methods.insert((name.to_string(), for_type), func);
    }

    /// Search for a method in the current and parent scopes
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

    /// A special case of get_var, for better error reporting
    pub fn get_fn(&self, name: &str) -> Function {
        let val = self.get_var(name).expect(&format!("unknown function {}", name));
        match val {
            Value::Function(f) => f,
            _ => panic!("{:?} is not a function", val)
        }
    }
}