use crate::Value;

// TODO: refactor into {res} | {sc {} | {}}
pub enum EvRes {
    Value(Value),
    ReturnSC(Value),
    BreakSC(Value),
}

impl EvRes {
    pub fn new_val(value: Value) -> Self {
        Self::Value(value)
    }

    pub fn is_short_circuit(&self) -> bool {
        match self {
            EvRes::Value(_) => false,
            _ => true,
        }
    }

    pub fn into_value(self) -> Option<Value> {
        match self {
            Self::Value(val) => Some(val),
            _ => None,
        }
    }
}
