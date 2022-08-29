use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Path {
    pub module: Name,
    pub item: Name,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Name(pub String);

impl From<String> for Name {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<Name> for String {
    fn from(input: Name) -> Self {
        let Name(result) = input;
        result
    }
}

impl<'s> From<&'s str> for Name {
    fn from(input: &'s str) -> Self {
        input.to_string().into()
    }
}

pub struct Import {
    pub module_name: Name,
    pub items: Vec<Name>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub items: Vec<Name>,
}

pub use expression::*;
mod expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FnDef {
    pub name: Name,
    pub parameters: Vec<Name>,
    pub expressions: Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopLevel {
    Export(Export),
    FnDef(FnDef),
}

impl TopLevel {
    pub fn as_fndef(&self) -> Option<&FnDef> {
        match self {
            Self::FnDef(result) => Some(result),
            _ => None,
        }
    }
    pub fn into_fndef(self) -> Option<FnDef> {
        match self {
            Self::FnDef(result) => Some(result),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub items: Vec<TopLevel>,
}
