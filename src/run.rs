use crate::parse::{AstNode,Term,Op};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    DoublePrecisionFloat(f64),
    Str(String),
    None,
    Undefined,
}

pub fn print_builtin(args: Vec<Value>) -> Value {
    let mut print_strs = vec![];
    for arg in args {
        let s = match arg {
            Value::DoublePrecisionFloat(x) => format!("{}", x),
            Value::Integer(x) => format!("{}", x),
            Value::Str(x) => format!("{}", x),
            Value::None => String::from("None"),
            Value::Undefined => String::from("undefined"),
        };
        print_strs.push(s);
    }
    let joined = print_strs.join(" ");
    println!("{}", joined);
    Value::None
}

pub fn fn_call(name: &str, args: &Vec<AstNode>, scope: &mut Scope) -> Value {
    let mut evalled_args = vec![];
    for arg in args {
        evalled_args.push(eval(arg, scope));
    }

    match name {
        "print" => print_builtin(evalled_args),
        _ => {
            let func = scope.get_fn(name);
            let mut inner_scope = scope.nest(format!("function \"{}\"", name));
            for (idx, argname) in func.args.iter().enumerate() {
                inner_scope.set_var(argname, evalled_args[idx].clone());
            }
            inner_scope.exec_fn(func)
            // For closures: check for variables in inner scope to be lifted back out?
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<AstNode>,
}

#[derive(Clone, Debug)]
pub struct Scope {
    context: String,
    fns: HashMap<String, Function>,
    vars: HashMap<String, Value>,
}
impl Scope {
    pub fn new(context: String) -> Scope {
        Scope{ context, fns: HashMap::new(), vars: HashMap::new() }
    }

    pub fn nest(&self, context: String) -> Scope  {
        let mut ret = self.clone();
        ret.context = context;
        ret
    }

    pub fn add_fn(&mut self, name: String, args: Vec<String>, body: Vec<AstNode>) {
        let func = Function { name: name.clone(), args, body };
        self.fns.insert(name, func);
    }

    pub fn get_fn(&self, name: &str) -> Function {
        let func = match self.fns.get(name) {
            Some(body) => body,
            None => panic!("unknown function: \"{}\" | context: {}", name, self.context),
        };
        return func.clone();
    }

    pub fn exec_fn(&mut self, func: Function) -> Value {
        let mut ret: Option<Value> = None;
        for ast in func.body {
            match ast {
                AstNode::Return(ast) => {
                    ret = Some(eval(&ast, self));
                    break;
                },
                _ => stmt(&ast, self)
            };
        }

        match ret {
            Some(val) => val,
            None => Value::None,
        }
    }

    pub fn set_var(&mut self, name: &str, val: Value) {
        self.vars.insert(name.to_string(), val);
    }

    pub fn get_var(&mut self, name: &str) -> Value {
        let val = match self.vars.get(name) {
            Some(x) => x,
            None => panic!("unknown var: \"{}\" | context: {}", name, self.context),
        };
        val.clone()
    }
}

pub fn stmt(ast: &AstNode, scope: &mut Scope) {
    match ast {
        AstNode::FnDef{ name, args, body } => {
            scope.add_fn(String::from(name), args.to_vec(), body.to_vec());
        },
        AstNode::Assignment(Term::Ident(var), astbox) => {
            let val = eval(astbox, scope);
            scope.set_var(var, val);
        }
        _ => { eval(ast, scope); },
    };
}

fn arithmetic(lhs: Value, op: Op, rhs: Value) -> Value {
    match lhs {
        Value::Integer(i) => arith_int(i, op, rhs),
        Value::DoublePrecisionFloat(f) => arith_float(f, op, rhs),
        Value::Str(s) => arith_str(s, op, rhs),
        Value::Undefined => panic!("Can't {:?} undefined and {:?}", op, rhs),
        Value::None => panic!("Can't {:?} None and {:?}", op, rhs),
    }
}

fn arith_int(a: i32, op: Op, rhs: Value) -> Value {
    let res = match rhs {
        Value::Integer(b) => match op {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
            Op::Shr => a >> b,
            Op::Shl => a << b,
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
            Op::Mod => a % b,
            Op::Exp => a.pow(b as u32),
        },
        _ => panic!("Can't {:?} int {} with {:?}", op, a, rhs),
    };
    Value::Integer(res)
}

fn arith_float(a: f64, op: Op, rhs: Value) -> Value {
    let res = match rhs {
        Value::DoublePrecisionFloat(b) => match op {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
            _ => panic!("Operation {:?} not defined for float", op),
        },
        _ => panic!("Can't {:?} float {} with {:?}", op, a, rhs),
    };
    Value::DoublePrecisionFloat(res)
}

pub fn arith_str(a: String, op: Op, rhs: Value) -> Value {
    let res = match rhs {
        Value::Str(b) => match op {
            Op::Add => a + &b,
            _ => panic!("operator {:?} not defined for string", op),
        },
        _ => panic!("Can't {:?} string {} with {:?}", op, a, rhs),
    };
    Value::Str(res)
}

pub fn eval(ast: &AstNode, scope: &mut Scope) -> Value {
    match ast {
        AstNode::FnCall{ name, args } => fn_call(name, args, scope),
        AstNode::Arithmetic(lhs, op, rhs) => arithmetic(eval(lhs, scope), op.clone(), eval(rhs, scope)),
        AstNode::Term(Term::Str(x)) => Value::Str(x.to_string()),
        AstNode::Term(Term::Integer(x)) => Value::Integer(*x),
        AstNode::Term(Term::DoublePrecisionFloat(x)) => Value::DoublePrecisionFloat(*x),
        AstNode::Term(Term::Ident(var)) => {
            scope.get_var(var)
        }
        _ => panic!("Unexpected ast {:?}", ast),
    }
}

pub fn run(ast_list: Vec<AstNode>) {
    let mut global_scope = Scope::new(String::from("<top level>"));
    for ast_node in ast_list {
        stmt(&ast_node, &mut global_scope);
    }
}