use serde::{Deserialize, Serialize};

use super::{Name, Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub expressions: Vec<BExpr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub variable_name: Name,
    pub value: BExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoke {
    pub variable_name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Litteral {
    String(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
    List(Vec<BExpr>),
    Map(Vec<(Name, BExpr)>),
}

impl From<String> for Litteral {
    fn from(input: String) -> Self {
        Self::String(input)
    }
}

impl From<&str> for Litteral {
    fn from(input: &str) -> Self {
        Self::String(input.to_string())
    }
}

impl From<i32> for Litteral {
    fn from(input: i32) -> Self {
        Self::Integer(input)
    }
}

impl From<f32> for Litteral {
    fn from(input: f32) -> Self {
        Self::Float(input)
    }
}

impl From<bool> for Litteral {
    fn from(bool: bool) -> Self {
        Self::Bool(bool)
    }
}

impl<T> From<Vec<T>> for Litteral
where
    T: Into<Litteral>,
{
    fn from(input: Vec<T>) -> Self {
        let list = input
            .into_iter()
            .map(|e| Box::new(Expr::Litteral(e.into())))
            .collect();
        Self::List(list)
    }
}

impl<S, T> From<Vec<(S, T)>> for Litteral
where
    S: ToString,
    T: Into<Litteral>,
{
    fn from(input: Vec<(S, T)>) -> Self {
        let list = input
            .into_iter()
            .map(|(n, e)| (n.to_string().into(), Box::new(Expr::Litteral(e.into()))))
            .collect();
        Self::Map(list)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FnCall {
    pub fn_path: Path,
    pub arguments: Vec<BExpr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition: BExpr,
    pub true_case: BExpr,
    pub false_case: BExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loop {
    pub body: BExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return {
    pub expression: BExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Break {
    pub expression: BExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Block(Block),
    Assignment(Assignment),
    Invoke(Invoke),
    Litteral(Litteral),
    FnCall(FnCall),
    Condition(Condition),
    Loop(Loop),
    Return(Return),
    Break(Break),
}

impl Expr {
    pub fn boxed(self) -> BExpr {
        Box::new(self)
    }
}

pub type BExpr = Box<Expr>;
