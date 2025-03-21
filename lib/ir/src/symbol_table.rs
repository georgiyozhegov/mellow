use std::collections::{hash_map::Iter, HashMap};

use syntax::parse::{
    expression::{self, BinaryKind, Expression},
    statement::Statement,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    I64,
    I32,
    String,
    Boolean,
}

macro_rules! in_range {
    ($value:expr, $type:ident) => {
        if $type::MIN as i128 <= $value && $value <= $type::MAX as i128 {
            true
        } else {
            false
        }
    };
}

pub fn type_of(expression: Expression) -> Type {
    match expression {
        Expression::Integer(node) => {
            return Type::I64;
        }
        Expression::Binary(node) => {
            let left = type_of(*node.left);
            let right = type_of(*node.right);
            if matches!(
                node.kind,
                BinaryKind::Equal | BinaryKind::Greater | BinaryKind::Less
            ) {
                if left != right {
                    panic!("invalid type of operands ({left:?} and {right:?})");
                }
                return Type::Boolean;
            }
            if left == right {
                return left;
            }
        }
        Expression::String(value) => {
            return Type::String;
        }
        Expression::If(expression::If {
            condition,
            if_,
            or,
            else_,
        }) => {
            let condition = type_of(*condition);
            if condition != Type::Boolean {
                panic!("invalid type of condition ({condition:?})");
            }
            // TODO: other checks
        }
        Expression::Boolean(value) => {
            return Type::Boolean;
        }
        _ => {}
    }
    panic!("invalid type");
}

#[derive(Debug)]
pub struct VariableMeta {
    pub mutable: bool,
    pub type_: Type,
}

#[derive(Debug)]
pub struct FunctionMeta {
    pub external: bool,
}

#[derive(Debug)]
pub struct SymbolTable {
    variables: HashMap<String, VariableMeta>,
    functions: HashMap<String, FunctionMeta>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

impl SymbolTable {
    pub fn insert_variable(&mut self, identifier: String, meta: VariableMeta) {
        self.variables.insert(identifier, meta);
    }

    pub fn get_variable(&mut self, identifier: &String) -> Option<&VariableMeta> {
        self.variables.get(identifier)
    }

    pub fn get_mut_variable(&mut self, identifier: &String) -> Option<&mut VariableMeta> {
        self.variables.get_mut(identifier)
    }

    pub fn variables(&self) -> Iter<String, VariableMeta> {
        self.variables.iter()
    }
}

impl SymbolTable {
    pub fn insert_function(&mut self, identifier: String, meta: FunctionMeta) {
        self.functions.insert(identifier, meta);
    }

    pub fn get_function(&mut self, identifier: &String) -> Option<&FunctionMeta> {
        self.functions.get(identifier)
    }

    pub fn get_mut_function(&mut self, identifier: &String) -> Option<&mut FunctionMeta> {
        self.functions.get_mut(identifier)
    }

    pub fn functions(&self) -> Iter<String, FunctionMeta> {
        self.functions.iter()
    }
}

pub fn construct(source: &Vec<Statement>) -> SymbolTable {
    let mut table = SymbolTable::new();

    table.insert_function("debug_i64".into(), FunctionMeta { external: true });

    for statement in source.iter() {
        match statement {
            Statement::Let(value) => {
                let type_ = type_of(value.value.clone());
                table.insert_variable(
                    value.identifier.clone(),
                    VariableMeta {
                        mutable: value.mutable,
                        type_,
                    },
                );
            }
            _ => {}
        }
    }
    table
}
