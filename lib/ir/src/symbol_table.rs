use std::collections::{HashMap, hash_map::Iter};

use mellow_parse::{
    VisitExpression, VisitStatement,
    tree::*,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    I64,
    I32,
    String,
    Boolean,
}

pub struct TypeChecker {
    table: SymbolTable,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
        }
    }
}

#[derive(Debug)]
pub struct TypeError(pub &'static str);

impl TypeChecker {
    pub fn construct(mut self, source: &Vec<Statement>) -> Result<SymbolTable, TypeError> {
        for statement in source {
            statement.clone().visit(&mut self, &mut ())?;
        }
        Ok(self.table)
    }
}

impl VisitStatement for TypeChecker {
    type Output = Result<(), TypeError>;
    type Context = ();

    fn let_(&mut self, node: Let, _context: &mut Self::Context) -> Self::Output {
        let meta = VariableMeta {
            mutable: node.mutable,
            type_: node.value.visit(self)?,
        };
        self.table.insert_variable(node.identifier, meta);
        Ok(())
    }

    fn assign(&mut self, node: Assign, _context: &mut Self::Context) -> Self::Output {
        if let Some(meta) = self.table.get_variable(&node.identifier) {
            if !meta.mutable {
                return Err(TypeError("cannot assign twice to an immutable variable"));
            }
            return Ok(());
        }
        Err(TypeError("variable is not found"))
    }
}

impl VisitExpression for TypeChecker {
    type Output = Result<Type, TypeError>;

    fn integer(&mut self, node: Integer) -> Self::Output {
        Ok(Type::I64) // TODO
    }

    fn identifier(&mut self, node: Identifier) -> Self::Output {
        if let Some(meta) = self.table.get_variable(&node.name) {
            return Ok(meta.type_.clone());
        }
        Err(TypeError("variable is not found"))
    }
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

pub fn construct(source: &Vec<Statement>) -> Result<SymbolTable, TypeError> {
    let type_checker = TypeChecker::new();
    let mut table = type_checker.construct(source)?;
    table.insert_function("debug_i64".into(), FunctionMeta { external: true });
    Ok(table)
}
