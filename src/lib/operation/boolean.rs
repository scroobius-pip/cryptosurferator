use crate::lib::operand::*;

//boolean operator that works on two values of the same type
pub enum BoolOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
    Xor,
    Not,
}

pub type BoolOperation = (BoolOperator, Operand, Operand);
