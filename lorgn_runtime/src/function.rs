use std::fmt::Debug;

use lorgn_lang::ast::{FnDef, Name, Path};

use crate::{runtime::Context, Value};

#[derive(Debug)]
pub struct Imported {
    _path: Path,
}

pub struct Native {
    arg_count: usize,
    handler: Box<dyn FnMut(Vec<Value>) -> Value>,
}

impl Native {
    pub fn run(&mut self, args: Vec<Value>) -> Value {
        if self.arg_count != args.len() {
            panic!("too few args")
        }
        (self.handler)(args)
    }
}

impl Debug for Native {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("<Native>").finish()
    }
}

#[derive(Debug)]
pub enum FnImpl {
    Imported(Imported),
    Defined(FnDef),
    Native(Native),
}

#[derive(Debug)]
pub struct Function {
    name: Name,
    implem: FnImpl,
}

impl Function {
    pub fn new_defined(name: Name, definition: FnDef) -> Self {
        let implem = FnImpl::Defined(definition);
        Self { name, implem }
    }

    pub fn new_imported(name: Name, module: Name) -> Self {
        let implem = FnImpl::Imported(Imported {
            _path: Path {
                item: name.clone(),
                module,
            },
        });
        Self { name, implem }
    }
    pub fn new_native<const N: usize>(
        name: Name,
        mut caller: impl FnMut([Value; N]) -> Value + 'static,
    ) -> Self {
        let handler = Box::new(move |values: Vec<Value>| {
            let casted = values.try_into().unwrap();
            (caller)(casted)
        }) as Box<dyn FnMut(Vec<Value>) -> Value>;
        Self {
            name,
            implem: FnImpl::Native(Native {
                arg_count: N,
                handler,
            }),
        }
    }

    pub fn call(&mut self, args: Vec<Value>, context: &mut Context) -> Value {
        match &mut self.implem {
            FnImpl::Defined(definition) => context.run_fun(definition, args),
            FnImpl::Native(native) => native.run(args),
            FnImpl::Imported(_imported) => {
                todo!() // let mut res = context.find_function(&imported.path).unwrap();
                        // res.call(args, context)
            }
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}
