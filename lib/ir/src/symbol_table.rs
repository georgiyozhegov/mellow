use std::collections::{hash_map::Iter, HashMap};

use syntax::tree::Statement;

#[derive(Debug)]
pub struct VariableMeta {
    pub mutable: bool,
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
            Statement::Let {
                identifier,
                mutable,
                ..
            } => {
                table.insert_variable(identifier.into(), VariableMeta { mutable: *mutable });
            }
            _ => {}
        }
    }
    table
}
