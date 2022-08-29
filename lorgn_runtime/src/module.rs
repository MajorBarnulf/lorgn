use std::{
    cell::{RefCell, RefMut},
    collections::{HashMap, HashSet},
};

use lorgn_lang::ast::{self, Name, TopLevel};

use crate::{Function, Value};

#[derive(Debug)]
pub struct Module {
    name: Name,
    functions: HashMap<Name, RefCell<Function>>,
    _exports: HashSet<Name>, // TODO
}

impl Module {
    pub fn new_empty(name: impl ToString) -> Self {
        let name = name.to_string().into();
        Self {
            _exports: HashSet::new(),
            functions: HashMap::new(),
            name,
        }
    }
    pub fn from_ast(name: impl ToString, content: ast::Module) -> Self {
        let name = name.to_string().into();
        let mut functions = HashMap::new();
        let mut exports = HashSet::new();

        for item in content.items {
            match item {
                TopLevel::Export(export) => export.items.iter().for_each(|e| {
                    exports.insert(e.clone());
                }),
                TopLevel::FnDef(fndef) => {
                    let name = fndef.name.clone();
                    let fun = Function::new_defined(name.clone(), fndef);
                    functions.insert(name, RefCell::new(fun));
                }
            };
        }

        Self {
            name,
            functions,
            _exports: exports,
        }
    }
    pub fn push_native<const N: usize>(
        &mut self,
        name: Name,
        caller: impl FnMut([Value; N]) -> Value + 'static,
    ) {
        let nat = Function::new_native(name.clone(), caller);
        self.functions.insert(name, RefCell::new(nat));
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn get_function(&self, name: &Name) -> Option<RefMut<Function>> {
        self.functions.get(name).map(|entry| entry.borrow_mut())
    }
}
