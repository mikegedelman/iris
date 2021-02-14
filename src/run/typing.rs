use crate::ast::AstNode;
use super::Value;
use super::scope::Scope;

/// A list of primitive types that should mirror the Value enum.
/// I don't like that these two store similar information, but one
/// is meant to store type information, and the other is the actual value.
/// Type information is already stored in Value due to it being an enum,
/// but this allows us to operate on that type info separately at runtime
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Integer,
    Str,
    Bool,
    List,
    Function,
    Option,
    // Enum,
    // Record,
    // more
}

/// TypeDefinition can store any concievable type in the system:
/// even when we support user types, they will simply be some
/// composition of our builtin types
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeDefinition {
    PrimitiveType(PrimitiveType),
    // CompoundType(Box<TypeDefinition>),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<AstNode>,
    pub scope: Scope,
}

/// Determine a type definition from the given string
/// Eventually this should support parsing complex type definitions
/// from the source
pub fn read_type_definition(s: &str) -> TypeDefinition {
    match s {
        "list" => TypeDefinition::PrimitiveType(PrimitiveType::List),
        _ => panic!("unknown type {:?}", s),
    }
}

/// Return the type of the given value
pub fn type_of(v: &Value) -> TypeDefinition {
    match v {
        Value::Integer(_) => TypeDefinition::PrimitiveType(PrimitiveType::Integer),
        Value::Str(_) => TypeDefinition::PrimitiveType(PrimitiveType::Str),
        Value::Bool(_) => TypeDefinition::PrimitiveType(PrimitiveType::Bool),
        Value::List(_) => TypeDefinition::PrimitiveType(PrimitiveType::List),
        Value::Function(_) => TypeDefinition::PrimitiveType(PrimitiveType::Function),
        Value::Some(_) => TypeDefinition::PrimitiveType(PrimitiveType::Option),
        Value::None => TypeDefinition::PrimitiveType(PrimitiveType::Option),
    }
}