use std::collections::HashMap;

use lorgn_lang::ast::{Expr, Name};

use crate::{Module, Value};

pub struct Runtime {
    modules: HashMap<Name, Module>,
}

impl Default for Runtime {
    fn default() -> Self {
        let modules = HashMap::new();
        Self { modules }
    }
}

impl Runtime {
    pub fn register(&mut self, module: Module) {
        self.modules.insert(module.name().clone(), module);
    }

    pub fn evaluate(&mut self, expression: Expr) -> Value {
        let mut context = self.context();
        context.run_expr(&expression)
    }

    fn context(&mut self) -> Context {
        Context::new(&mut self.modules)
    }
}

pub use eval_result::EvRes;
mod eval_result;

pub struct Scope {
    bubble_variables: bool,
    variables: HashMap<Name, Value>,
}

impl Scope {
    pub fn new(bubble_variables: bool) -> Self {
        let variables = HashMap::new();
        Self {
            bubble_variables,
            variables,
        }
    }

    pub fn new_with(variables: Vec<(Name, Value)>, bubble_variables: bool) -> Self {
        let mut result = Self::new(bubble_variables);
        for (name, value) in variables {
            result.insert(name, value)
        }
        result
    }

    pub fn insert(&mut self, name: Name, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &Name) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn get_mut(&mut self, name: &Name) -> Option<&mut Value> {
        self.variables.get_mut(name)
    }
}

pub use context::Context;
mod context {
    use std::{cell::RefMut, collections::HashMap};

    use lorgn_lang::ast::{
        Assignment, Block, Break, Condition, Expr, FnCall, FnDef, Invoke, Litteral, Loop, Name,
        Path, Return,
    };

    use crate::{Function, Module, Value};

    use super::{EvRes, Scope};

    pub struct Context<'r> {
        modules: &'r HashMap<Name, Module>,
        scopes: Vec<Scope>,
    }

    impl<'r> Context<'r> {
        pub fn new(modules: &'r mut HashMap<Name, Module>) -> Self {
            let scopes = vec![];
            Self { modules, scopes }
        }

        pub fn find_function(&self, path: &Path) -> Option<RefMut<Function>> {
            self.modules
                .get(&path.module)
                .and_then(|m| m.get_function(&path.item))
        }

        pub fn find_variable(&mut self, name: &Name) -> Option<&mut Value> {
            for scope in self.scopes.iter_mut().rev() {
                if !scope.bubble_variables {
                    return None;
                }
                if let Some(result) = scope.variables.get_mut(name) {
                    return Some(result);
                }
            }
            None
        }

        pub fn run_fun(&mut self, fn_def: &FnDef, params: Vec<Value>) -> Value {
            let variables = fn_def
                .parameters
                .iter()
                .cloned()
                .zip(params.into_iter())
                .collect();
            self.push_scope(Scope::new_with(variables, false));
            let res = self.eval_block(&fn_def.expressions);
            self.pop_scope();
            match res {
                EvRes::Value(res) => res,
                EvRes::ReturnSC(res) => res,
                EvRes::BreakSC(_) => panic!("break outside of loop"),
            }
        }

        pub fn run_expr(&mut self, expr: &Expr) -> Value {
            self.eval_expr(expr).into_value().unwrap()
        }

        fn push_scope(&mut self, scope: Scope) {
            self.scopes.push(scope)
        }

        fn pop_scope(&mut self) {
            self.scopes.pop();
        }

        pub fn top_scope(&mut self) -> Option<&mut Scope> {
            self.scopes.last_mut()
        }

        pub fn top_index(&self) -> usize {
            self.scopes.len() - 1
        }

        fn eval_expr(&mut self, expr: &Expr) -> EvRes {
            match expr {
                Expr::Block(block) => self.eval_block(block),
                Expr::Assignment(assignment) => self.eval_assignment(assignment),
                Expr::Invoke(invoke) => self.eval_invoke(invoke),
                Expr::Litteral(litteral) => self.eval_litteral(litteral),
                Expr::FnCall(fn_call) => self.eval_fn_call(fn_call),
                Expr::Condition(condition) => self.eval_condition(condition),
                Expr::Loop(loop_) => self.eval_loop(loop_),
                Expr::Return(return_) => self.eval_return(return_),
                Expr::Break(break_) => self.eval_break(break_),
            }
        }

        fn eval_block(&mut self, block: &Block) -> EvRes {
            let mut last = None;
            for expr in &block.expressions {
                let result = self.eval_expr(expr);
                if let EvRes::Value(result) = result {
                    last = Some(result);
                } else {
                    return result;
                }
            }
            let result = last.unwrap_or(Value::None);
            EvRes::new_val(result)
        }

        fn eval_assignment(&mut self, assignment: &Assignment) -> EvRes {
            let result = self.eval_expr(&assignment.value);
            if let EvRes::Value(result) = result {
                let name = assignment.variable_name.clone();
                self.top_scope().unwrap().insert(name, result.clone());
                EvRes::new_val(result)
            } else {
                result
            }
        }

        fn eval_invoke(&mut self, invoke: &Invoke) -> EvRes {
            let value = self.find_variable(&invoke.variable_name).unwrap().clone();
            EvRes::new_val(value)
        }

        fn eval_litteral(&mut self, litteral: &Litteral) -> EvRes {
            match litteral {
                Litteral::String(str) => EvRes::Value(str.clone().into()),
                Litteral::Integer(int) => EvRes::Value((*int).into()),
                Litteral::Float(flt) => EvRes::Value((*flt).into()),
                Litteral::Bool(bool) => EvRes::Value((*bool).into()),
                Litteral::List(vec) => {
                    let mut results = vec![];
                    for expr in vec {
                        let result = self.eval_expr(expr);
                        if let EvRes::Value(result) = result {
                            results.push(result);
                        } else {
                            return result;
                        }
                    }
                    EvRes::Value(results.into())
                }
                Litteral::Map(map) => {
                    let mut results = HashMap::new();
                    for (name, expr) in map {
                        let result = self.eval_expr(expr);
                        if let EvRes::Value(result) = result {
                            results.insert(name.clone(), result);
                        } else {
                            return result;
                        }
                    }
                    EvRes::Value(results.into())
                }
            }
        }

        fn eval_fn_call(&mut self, fn_call: &FnCall) -> EvRes {
            let mut args = vec![];
            for arg in &fn_call.arguments {
                let res = self.eval_expr(arg);
                if res.is_short_circuit() {
                    return res;
                }
                args.push(res.into_value().unwrap());
            }
            let path = &fn_call.fn_path;
            let module = self.modules.get(&path.module).unwrap();
            let mut function = module.get_function(&path.item).unwrap();
            let res = function.call(args, self);
            EvRes::Value(res)
        }

        fn eval_condition(&mut self, condition: &Condition) -> EvRes {
            let cond = self.eval_expr(&condition.condition);
            if cond.is_short_circuit() {
                return cond;
            }
            let cond = cond.into_value().unwrap();
            if cond.into_bool().unwrap() {
                self.eval_expr(&condition.true_case)
            } else {
                self.eval_expr(&condition.false_case)
            }
        }

        fn eval_loop(&mut self, loop_: &Loop) -> EvRes {
            let body = &loop_.body;
            let result = loop {
                let res = self.eval_expr(body);
                match res {
                    EvRes::ReturnSC(_) => return res,
                    EvRes::BreakSC(result) => break result,
                    _ => (),
                };
            };
            EvRes::new_val(result)
        }

        fn eval_return(&mut self, return_: &Return) -> EvRes {
            let result = self.eval_expr(&return_.expression);
            match result {
                EvRes::ReturnSC(_) => result,
                EvRes::Value(v) => EvRes::ReturnSC(v),
                EvRes::BreakSC(_) => panic!("break outside of loop"),
            }
        }

        fn eval_break(&mut self, break_: &Break) -> EvRes {
            let result = self.eval_expr(&break_.expression);
            match result {
                EvRes::ReturnSC(_) => result,
                EvRes::Value(v) => EvRes::BreakSC(v),
                EvRes::BreakSC(v) => EvRes::BreakSC(v),
            }
        }
    }
}
