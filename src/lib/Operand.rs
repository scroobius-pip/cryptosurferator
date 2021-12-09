use crate::lib::operation::trade::*;
use crate::lib::operation::*;
use crate::lib::terminal_type::*;
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
    ) -> TerminalType {
        match self {
            Operand::Pointer(pointer) => {
                let operation = &operation_list[*pointer];
                operation.evaluate(operation_list, trade_list)
            }
            Operand::Terminal(terminal) => terminal.clone(),
            Operand::None => TerminalType::Number(0.0),
        }
    }
}
