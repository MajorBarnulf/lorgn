use std::collections::HashMap;

use gc::Gc;
use gc_derive::{Finalize, Trace};
use lorgn_lang::ast::Name;

#[derive(Debug, Clone, Trace, Finalize)]
pub struct InnerObj(#[unsafe_ignore_trace] HashMap<Name, Value>);

#[derive(Debug, Clone, Trace, Finalize)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
    List(Gc<Vec<Value>>),
    Object(Gc<InnerObj>),
    None,
}

impl Value {
    pub fn into_string(self) -> Option<String> {
        match &self {
            Self::String(str) => Some(str.clone()),
            _ => None,
        }
    }
    pub fn into_i32(self) -> Option<i32> {
        match self {
            Self::Integer(int) => Some(int),
            _ => None,
        }
    }
    pub fn into_bool(self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(bool),
            _ => None,
        }
    }
}

impl From<String> for Value {
    fn from(input: String) -> Self {
        Self::String(input)
    }
}

impl From<i32> for Value {
    fn from(input: i32) -> Self {
        Self::Integer(input)
    }
}

impl From<f32> for Value {
    fn from(input: f32) -> Self {
        Self::Float(input)
    }
}

impl From<bool> for Value {
    fn from(input: bool) -> Self {
        Self::Bool(input)
    }
}

impl From<Vec<Value>> for Value {
    fn from(input: Vec<Value>) -> Self {
        let gc = Gc::new(input);
        Self::List(gc)
    }
}

impl From<HashMap<Name, Value>> for Value {
    fn from(input: HashMap<Name, Value>) -> Self {
        let gc = Gc::new(InnerObj(input));
        Self::Object(gc)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(input: Option<T>) -> Self {
        match input {
            Some(input) => input.into(),
            None => Self::None,
        }
    }
}
