use crate::lib::op::operand::*;
pub enum IndexOperator {
    Last,
    First,
    Operand(Operand),
}

pub type IndexOperation = (IndexOperator, Operand);
