pub mod boolean;
pub mod branch;
pub mod constant;
pub mod index;
pub mod market_data;
pub mod num_pick;
pub mod number;
pub mod trade;

use crate::lib::Operand::*;
use crate::lib::TerminalType::*;
use boolean::*;
use branch::*;
use constant::*;
use index::*;
use market_data::*;
use num_pick::*;
use number::*;
use std::default::Default;
use trade::*;

pub enum Operation {
    Branch(BranchOperation),
    Bool(BoolOperation),
    Trade(TradeOperation),
    MarketData(MarketDataOperation),
    NumPick(NumPickOperation),
    Number(NumOperation),
    Constant(ConstantOperation),
    Index(IndexOperation),
}

impl Operation {
    pub fn evaluate(
        &self,
        operation_list: &OperationList,
        trade_list: &mut TradeList,
    ) -> TerminalType {
        match self {
            Operation::Index((operand_left, operand_right)) => {
                let index = operand_left.evaluate(operation_list, trade_list).to_f32() as usize;
                let list = operand_right.evaluate(operation_list, trade_list).to_list();
                //check if index is out of bounds if so use last element
                if index >= list.len() {
                    TerminalType::Number(list[list.len() - 1])
                } else {
                    TerminalType::Number(list[index])
                }
            }
            Operation::Constant((operator, operand)) => {
                let market_index = operand.evaluate(operation_list, trade_list);
                match operator {
                    ConstantOperator::CurrentMarketPrice => TerminalType::Number(0.0),
                    ConstantOperator::PortfolioValue => TerminalType::Number(0.0),
                }
            }
            Operation::Number((operator, operand_left, operand_right)) => {
                let left = operand_left.evaluate(operation_list, trade_list);
                let right = operand_right.evaluate(operation_list, trade_list);
                TerminalType::Number(operator.func()(left.to_f32(), right.to_f32()))
            }

            Operation::Trade((
                operator,
                market_index,
                market_price,
                market_amount,
                trade_leverage,
            )) => {
                let market_index =
                    market_index.evaluate(operation_list, trade_list).to_f32() as usize;
                let market_price = market_price.evaluate(operation_list, trade_list).to_f32();
                let market_amount = market_amount.evaluate(operation_list, trade_list).to_f32();
                trade_list.push(trade::Trade {
                    operator: *operator,
                    index: market_index,
                    price: market_price,
                    amount: market_amount,
                    leverage: *trade_leverage,
                });
                TerminalType::Number(market_index as f32)
            }
            Operation::Bool((operator, operand_left, operand_right)) => {
                let left_value = operand_left.evaluate(operation_list, trade_list);
                let right_value = operand_right.evaluate(operation_list, trade_list);
                match operator {
                    BoolOperator::Equal => {
                        TerminalType::Number((left_value == right_value) as i32 as f32)
                    }
                    BoolOperator::NotEqual => {
                        TerminalType::Number((left_value != right_value) as i32 as f32)
                    }
                    BoolOperator::GreaterThan => {
                        TerminalType::Number((left_value > right_value) as i32 as f32)
                    }
                    BoolOperator::LessThan => {
                        TerminalType::Number((left_value < right_value) as i32 as f32)
                    }
                    BoolOperator::GreaterThanOrEqual => {
                        TerminalType::Number((left_value >= right_value) as i32 as f32)
                    }
                    BoolOperator::LessThanOrEqual => {
                        TerminalType::Number((left_value <= right_value) as i32 as f32)
                    }
                    BoolOperator::And => TerminalType::Number(
                        (left_value.to_bool() && right_value.to_bool()) as i32 as f32,
                    ),
                    BoolOperator::Or => TerminalType::Number(
                        (left_value.to_bool() || right_value.to_bool()) as i32 as f32,
                    ),
                    BoolOperator::Not => {
                        TerminalType::Number((!left_value.to_bool()) as i32 as f32)
                    }
                    BoolOperator::Xor => TerminalType::Number(
                        (left_value.to_bool() ^ right_value.to_bool()) as i32 as f32,
                    ),
                }
            }
            Operation::Branch((operand_operator, operand_left, operand_right)) => {
                match operand_operator {
                    Operand::Pointer(pointer) => {
                        let operand_operator_value = &operation_list
                            .get(*pointer)
                            .unwrap_or(&Operation::Number((
                                NumOperator::Add,
                                Operand::None,
                                Operand::None,
                            )))
                            .evaluate(operation_list, trade_list);

                        operand_operator_value.evaluate_branch_terminal(
                            operand_left,
                            operand_right,
                            operation_list,
                            trade_list,
                        )
                    }
                    Operand::Terminal(terminal) => terminal.evaluate_branch_terminal(
                        operand_left,
                        operand_right,
                        operation_list,
                        trade_list,
                    ),
                    Operand::None => TerminalType::Number(0.0),
                }
            }
            Operation::MarketData((
                market_data_operator,
                market_index_operand,
                data_index_start_operand,
                data_index_stop_operand,
                market_interval,
            )) => {
                let market_index_value = market_index_operand.evaluate(operation_list, trade_list);
                let data_index_start_value =
                    data_index_start_operand.evaluate(operation_list, trade_list);
                let data_index_stop_value =
                    data_index_stop_operand.evaluate(operation_list, trade_list);
                let market_data = get_market_data(
                    market_index_value.to_f32() as usize,
                    data_index_start_value.to_f32() as usize,
                    data_index_stop_value.to_f32() as usize,
                    *market_interval,
                );
                match market_data_operator {
                    MarketDataOperator::Open => TerminalType::NumberList(market_data.open),
                    MarketDataOperator::High => TerminalType::NumberList(market_data.high),
                    MarketDataOperator::Low => TerminalType::NumberList(market_data.low),
                    MarketDataOperator::Close => TerminalType::NumberList(market_data.close),
                    MarketDataOperator::Volume => TerminalType::NumberList(market_data.volume),
                    MarketDataOperator::OrderBookAsks => TerminalType::NumberList(market_data.asks),
                    MarketDataOperator::OrderBookBids => TerminalType::NumberList(market_data.bids),
                    MarketDataOperator::TradeCount => {
                        TerminalType::NumberList(market_data.trade_count)
                    }
                }
            }
            Operation::NumPick((num_pick_operator, operand)) => {
                let operand_value = operand.evaluate(operation_list, trade_list).to_list();
                let function = get_function_by_num_pick_operator(num_pick_operator);
                TerminalType::Number(function(operand_value))
            }
        }
    }
}

pub type OperationList = Vec<Operation>;
