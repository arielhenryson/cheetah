use super::function::Function;

pub type Value = ValueData;
pub type HeapValue = Box<Value>;

#[derive(Debug, Clone)]
pub enum ValueData {
    Undefined,

    Integer(i32),

    Function(Box<Function>),

    Array(
        Vec<Value>
    )
}

impl ValueData {
    pub fn is_truthy(&self) -> bool {
        match *self {
            ValueData::Integer(n) if n != 0 => true,
            _                               => false,
        }
    }
}