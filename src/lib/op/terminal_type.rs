use crate::lib::op::operand::*;
use crate::lib::op::operation::trade::TradeList;
use crate::lib::op::operation::*;
use std::cmp::Ordering;

#[derive(Clone, Debug)]

pub enum TerminalType {
    Number(f32),
    NumberList(Vec<f32>),
}

impl TerminalType {
    pub fn to_f32(&self) -> f32 {
        match self {
            TerminalType::Number(n) => *n as f32,
            TerminalType::NumberList(n) => n.len() as f32,
        }
    }
    pub fn to_bool(&self) -> bool {
        match self {
            TerminalType::Number(n) => *n > 0.0,
            TerminalType::NumberList(n) => n.len() > 0,
        }
    }
    pub fn to_list(&self) -> Vec<f32> {
        match self {
            TerminalType::Number(n) => vec![*n as f32],
            TerminalType::NumberList(n) => n.clone(),
        }
    }

    pub fn evaluate_branch_terminal(
        &self,
        operand_left: &Operand,
        operand_right: &Operand,
        operation_list: &OperationList,
        trade_list: &mut TradeList,
        context: &Context,
    ) -> TerminalType {
        let terminal_value = self.to_bool();

        if terminal_value {
            operand_left.evaluate(operation_list, trade_list, context)
        } else {
            operand_right.evaluate(operation_list, trade_list, context)
        }
    }
}

impl PartialEq for TerminalType {
    fn eq(&self, other: &Self) -> bool {
        self.to_f32() == other.to_f32()
    }
}

impl PartialOrd for TerminalType {
    fn partial_cmp(&self, other: &TerminalType) -> Option<Ordering> {
        match (self, other) {
            (TerminalType::Number(n), TerminalType::Number(m)) => n.partial_cmp(m),
            (TerminalType::NumberList(n), TerminalType::NumberList(m)) => n.partial_cmp(m),
            (TerminalType::NumberList(n), TerminalType::Number(m)) => {
                (n.len() as f32).partial_cmp(m)
            }
            (TerminalType::Number(n), TerminalType::NumberList(m)) => {
                n.partial_cmp(&(m.len() as f32))
            }
        }
    }
}
