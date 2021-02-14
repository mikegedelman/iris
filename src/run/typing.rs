use crate::ast::AstNode;
use super::Value;
use super::scope::Scope;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Integer,
    Str,
    Bool,
    List,
    Function,
    Enum,
    Option,
    // Record,
    // more
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeDefinition {
    PrimitiveType(PrimitiveType),
    CompoundType(Box<TypeDefinition>),
}

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct Enum {
//     name: String,
//     variants: Vec<String>,
// }

#[derive(Clone, Debug)]
pub struct Type {
    name: String,
    definition: TypeDefinition,
}


#[derive(Clone, Debug)]
pub struct Variable {
    type_: Type,
    val: Value,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<AstNode>,
    pub scope: Scope,
}

pub fn read_type_definition(s: &str) -> TypeDefinition {
    match s {
        "list" => TypeDefinition::PrimitiveType(PrimitiveType::List),
        _ => panic!("unknown type {:?}", s),
    }
}

pub fn type_of(v: &Value) -> TypeDefinition {
    match v {
        Value::Integer(_) => TypeDefinition::PrimitiveType(PrimitiveType::Integer),
        Value::Str(_) => TypeDefinition::PrimitiveType(PrimitiveType::Str),
        Value::Bool(_) => TypeDefinition::PrimitiveType(PrimitiveType::Bool),
        Value::List(_) => TypeDefinition::PrimitiveType(PrimitiveType::List),
        Value::Function(_) => TypeDefinition::PrimitiveType(PrimitiveType::Function),
        _ => panic!("Can't resolve type of value {:?}", v),
    }
}