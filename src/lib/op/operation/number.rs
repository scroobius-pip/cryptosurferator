use crate::lib::op::operand::*;

pub enum NumOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Min,
    Max,
    Cos,
    Sin,
    Tan,
    Pow,
    Log,
}

impl NumOperator {
    pub fn func(&self) -> fn(f32, f32) -> f32 {
        match self {
            NumOperator::Add => |a, b| a + b,
            NumOperator::Subtract => |a, b| a - b,
            NumOperator::Multiply => |a, b| a * b,
            NumOperator::Divide => |a, b| a / b,
            NumOperator::Modulo => |a, b| a % b,
            NumOperator::Min => |a, b| a.min(b),
            NumOperator::Max => |a, b| a.max(b),
            NumOperator::Cos => |a, _| a.cos(),
            NumOperator::Sin => |a, _| a.sin(),
            NumOperator::Tan => |a, _| a.tan(),
            NumOperator::Pow => |a, b| a.powf(b),
            NumOperator::Log => |a, b| a.log(b),
        }
    }
}

pub type NumOperation = (NumOperator, Operand, Operand);
