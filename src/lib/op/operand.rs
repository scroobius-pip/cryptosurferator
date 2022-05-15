use crate::lib::op::operation::trade::*;
use crate::lib::op::operation::*;
use crate::lib::op::terminal_type::*;

use super::environment::Env;
pub enum Operand {
    Pointer(usize),
    Terminal(TerminalType),
    None,
}

impl Operand {
    pub fn evaluate(
        &self,
        operation_list: &OperationList,
        trade_list: &mut TradeList,
        context: &Context,
        environment: &impl Env
    ) -> TerminalType {
        match self {
            Operand::Pointer(pointer) => {
                let operation = &operation_list[*pointer];
                operation.evaluate(operation_list, trade_list, context,environment)
            }
            Operand::Terminal(terminal) => terminal.clone(),
            Operand::None => TerminalType::Number(0.0),
        }
    }
}
