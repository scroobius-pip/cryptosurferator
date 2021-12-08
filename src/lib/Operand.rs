use crate::lib::Operation::Trade::*;
use crate::lib::Operation::*;
use crate::lib::TerminalType::*;
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

fn evaluate_pointer(
    pointer: &usize,
    operation_list: &OperationList,
    trade_list: &mut TradeList,
) -> TerminalType {
    let operation = &operation_list[*pointer];
    operation.evaluate(operation_list, trade_list)
}
