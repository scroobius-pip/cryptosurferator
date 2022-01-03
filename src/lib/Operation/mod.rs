pub mod boolean;
pub mod branch;
pub mod constant;
pub mod index;
pub mod market_data;
pub mod market_sort;
pub mod num_pick;
pub mod number;
pub mod trade;
use crate::lib::operand::*;
use crate::lib::terminal_type::*;
use boolean::*;
use branch::*;
use constant::*;
use index::*;
use market_data::*;
use market_sort::*;
use num_pick::*;
use number::*;
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
    Identity(Operand),
    MarketSort(MarketSortOperation),
}

pub type Context = Option<TerminalType>;
pub type OperationList = Vec<Operation>;

impl Operation {
    pub fn evaluate(
        &self,
        operation_list: &OperationList,
        trade_list: &mut TradeList,
        context: &Context,
    ) -> TerminalType {
        match self {
            Operation::MarketSort((operand,)) => {
                let mut market_index_list = vec![0.0, 1.0, 2.0, 3.0];
                market_index_list.sort_by(|market_index_a, market_index_b| {
                    let value_a = operand
                        .evaluate(
                            operation_list,
                            trade_list,
                            &Some(TerminalType::Number(*market_index_a)),
                        )
                        .to_f32(); //call with market_index_a
                    let value_b = operand
                        .evaluate(
                            operation_list,
                            trade_list,
                            &Some(TerminalType::Number(*market_index_b)),
                        )
                        .to_f32(); //call with market_index_b
                    value_a.partial_cmp(&value_b).unwrap()
                });
                TerminalType::NumberList(market_index_list)
            }
            Operation::Identity(operand) => operand.evaluate(operation_list, trade_list, context),
            Operation::Index((operand_left, operand_right)) => {
                let index = operand_left
                    .evaluate(operation_list, trade_list, context)
                    .to_f32() as usize;
                let list = operand_right
                    .evaluate(operation_list, trade_list, context)
                    .to_list();
                //check if index is out of bounds if so use last element
                if index >= list.len() {
                    TerminalType::Number(list[list.len() - 1])
                } else {
                    TerminalType::Number(list[index])
                }
            }
            Operation::Constant((operator, operand)) => {
                // let market_index = operand.evaluate(operation_list, trade_list);
                match operator {
                    ConstantOperator::MarketPrice => TerminalType::Number(0.0),
                    ConstantOperator::PortfolioValue => TerminalType::Number(0.0),
                    ConstantOperator::Zero => TerminalType::Number(0.0),
                    ConstantOperator::One => TerminalType::Number(1.0),
                    ConstantOperator::Two => TerminalType::Number(2.0),
                    ConstantOperator::Three => TerminalType::Number(3.0),
                    ConstantOperator::Four => TerminalType::Number(4.0),
                    ConstantOperator::Five => TerminalType::Number(5.0),
                    ConstantOperator::Six => TerminalType::Number(6.0),
                    ConstantOperator::Seven => TerminalType::Number(7.0),
                    ConstantOperator::Eight => TerminalType::Number(8.0),
                    ConstantOperator::Nine => TerminalType::Number(9.0),
                    ConstantOperator::Ten => TerminalType::Number(10.0),
                    ConstantOperator::PI => TerminalType::Number(3.141592653589793),
                    ConstantOperator::GoldenRatio => TerminalType::Number(1.618033988749895),
                    ConstantOperator::EulerNumber => TerminalType::Number(2.718281828459045),
                    _ => TerminalType::Number(0.0),
                }
            }
            Operation::Number((operator, operand_left, operand_right)) => {
                let left = operand_left.evaluate(operation_list, trade_list, context);
                let right = operand_right.evaluate(operation_list, trade_list, context);
                TerminalType::Number(operator.func()(left.to_f32(), right.to_f32()))
            }

            Operation::Trade((
                operator,
                market_index,
                market_price,
                market_amount,
                trade_leverage,
            )) => {
                let market_index = market_index
                    .evaluate(operation_list, trade_list, context)
                    .to_f32() as usize;
                let market_price = market_price
                    .evaluate(operation_list, trade_list, context)
                    .to_f32();
                let market_amount = market_amount
                    .evaluate(operation_list, trade_list, context)
                    .to_f32();
                trade_list.push(trade::Trade {
                    operator: *operator,
                    index: market_index,
                    price: market_price,
                    amount: market_amount,
                    leverage: *trade_leverage,
                });
                TerminalType::Number(1.0)
            }
            Operation::Bool((operator, operand_left, operand_right)) => {
                let left_value = operand_left.evaluate(operation_list, trade_list, context);
                let right_value = operand_right.evaluate(operation_list, trade_list, context);
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
                            .unwrap_or(&Operation::Identity(Operand::Terminal(
                                TerminalType::Number(0.0),
                            )))
                            .evaluate(operation_list, trade_list, context);

                        operand_operator_value.evaluate_branch_terminal(
                            operand_left,
                            operand_right,
                            operation_list,
                            trade_list,
                            context,
                        )
                    }
                    Operand::Terminal(terminal) => terminal.evaluate_branch_terminal(
                        operand_left,
                        operand_right,
                        operation_list,
                        trade_list,
                        context,
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
                //if context exists use it instead of market_index_operand
                // market_index_operand will only be evaluated normally if there is no context and itself is Operand::None
                // if there is a context, it will be used if it market_index_operand == Operand::None
                let market_index_value = match (market_index_operand, context) {
                    (Operand::None, Context::Some(context_terminal_type)) => context_terminal_type.clone(),
                    _ => market_index_operand.evaluate(operation_list, trade_list, context),
                };

                let data_index_start_value =
                    data_index_start_operand.evaluate(operation_list, trade_list, context);
                let data_index_stop_value =
                    data_index_stop_operand.evaluate(operation_list, trade_list, context);
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
                let operand_value = operand
                    .evaluate(operation_list, trade_list, context)
                    .to_list();
                let function = get_function_by_num_pick_operator(num_pick_operator);
                TerminalType::Number(function(operand_value))
            }
        }
    }
}

//tests
#[cfg(test)]
#[test]
fn test_evaluate_operation() {
    let context = None;
    let mut operation_list = OperationList::new();
    let mut trade_list = TradeList::new();
    let operation = Operation::Number((NumOperator::Add, Operand::Pointer(0), Operand::Pointer(1)));
    operation_list.push(Operation::Constant((ConstantOperator::One, Operand::None)));
    operation_list.push(Operation::Constant((ConstantOperator::Two, Operand::None)));
    let terminal_type = operation.evaluate(&operation_list, &mut trade_list, &context);
    assert_eq!(terminal_type, TerminalType::Number(3.0));
}

#[test]
fn test_evaluate_operation_number_constants() {
    let context = None;
    let operation_list = OperationList::new();
    let mut trade_list = TradeList::new();
    let operation = Operation::Number((
        NumOperator::Add,
        Operand::Terminal(TerminalType::Number(1.0)),
        Operand::Terminal(TerminalType::Number(2.0)),
    ));
    let terminal_type = operation.evaluate(&operation_list, &mut trade_list, &context);
    assert_eq!(terminal_type, TerminalType::Number(3.0));
}

#[test]
fn test_bool_operation() {
    let context = None;
    let operation_list = OperationList::new();
    let mut trade_list = TradeList::new();
    let operation = Operation::Bool((
        BoolOperator::Equal,
        Operand::Terminal(TerminalType::Number(1.0)),
        Operand::Terminal(TerminalType::Number(2.0)),
    ));
    let terminal_type = operation.evaluate(&operation_list, &mut trade_list, &context);
    assert_eq!(terminal_type, TerminalType::Number(0.0));
}
#[test]
fn test_branch_operation() {
    let context = None;
    let mut operation_list = OperationList::new();
    let mut operation_list_2 = OperationList::new();
    let mut trade_list = TradeList::new();
    let operation = Operation::Branch((
        Operand::Pointer(0),
        Operand::Terminal(TerminalType::Number(1.0)),
        Operand::Terminal(TerminalType::Number(2.0)),
    ));
    operation_list.push(Operation::Bool((
        BoolOperator::Equal,
        Operand::Terminal(TerminalType::Number(1.0)),
        Operand::Terminal(TerminalType::Number(2.0)),
    )));
    operation_list_2.push(Operation::Bool((
        BoolOperator::LessThan,
        Operand::Terminal(TerminalType::Number(1.0)),
        Operand::Terminal(TerminalType::Number(2.0)),
    )));
    let terminal_type = operation.evaluate(&operation_list, &mut trade_list, &context);
    assert_eq!(terminal_type, TerminalType::Number(2.0));

    let terminal_type = operation.evaluate(&operation_list_2, &mut trade_list, &context);
    assert_eq!(terminal_type, TerminalType::Number(1.0));
}

#[test]
fn test_num_pick_operation() {
    let context = None;
    let operation_list = OperationList::new();
    let mut trade_list = TradeList::new();
    let operation = Operation::NumPick((
        NumPickOperator::Max,
        Operand::Terminal(TerminalType::NumberList(vec![1.0, 2.0, 3.0])),
    ));
    let terminal_type = operation.evaluate(&operation_list, &mut trade_list, &context);
    assert_eq!(terminal_type, TerminalType::Number(3.0));
}

#[test]
fn test_num_pic_with_branch_operation() {
    let context = None;
    let mut trade_list = TradeList::new();
    let operation_list = vec![
        Operation::NumPick((
            NumPickOperator::Max,
            Operand::Terminal(TerminalType::NumberList(vec![1.0, 2.0, 3.0])),
        )),
        Operation::Bool((
            BoolOperator::GreaterThan,
            Operand::Pointer(0),
            Operand::Terminal(TerminalType::Number(2.0)),
        )),
        Operation::Branch((
            Operand::Pointer(1),
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(2.0)),
        )),
    ];

    let terminal_type = operation_list[operation_list.len() - 1].evaluate(
        &operation_list,
        &mut trade_list,
        &context,
    );
    assert_eq!(terminal_type, TerminalType::Number(1.0));
}
#[test]
fn test_trade_operation() {
    let context = None;
    let mut trade_list = TradeList::new();
    let operation_list = vec![
        Operation::Trade((
            TradeOperator::Buy,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(2.0)),
            Operand::Terminal(TerminalType::Number(3.0)),
            TradeLeverage::X1,
        )),
        Operation::Trade((
            TradeOperator::Sell,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(2.0)),
            Operand::Terminal(TerminalType::Number(3.0)),
            TradeLeverage::X1,
        )),
        Operation::MarketData((
            MarketDataOperator::High,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(1.0)),
            MarketDataInterval::Hour2,
        )),
        Operation::NumPick((NumPickOperator::Average, Operand::Pointer(2))),
        Operation::NumPick((NumPickOperator::Max, Operand::Pointer(2))),
        Operation::Bool((
            BoolOperator::LessThan,
            Operand::Pointer(4),
            Operand::Pointer(3),
        )),
        Operation::Branch((
            Operand::Pointer(5),
            Operand::Pointer(0),
            Operand::Pointer(1),
        )),
    ];

    operation_list[operation_list.len() - 1].evaluate(&operation_list, &mut trade_list, &context);

    assert_eq!(trade_list.len(), 1);
    assert_eq!(trade_list[0].operator, TradeOperator::Sell);
}
#[test]
fn test_multiple_bool_operation() {
    let context = None;
    let mut trade_list = TradeList::new();
    let operation_list = vec![
        Operation::Bool((
            BoolOperator::LessThan,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(2.0)),
        )),
        Operation::Bool((
            BoolOperator::Equal,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(1.0)),
        )),
        Operation::MarketData((
            MarketDataOperator::High,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(1.0)),
            MarketDataInterval::Hour2,
        )),
        Operation::NumPick((NumPickOperator::Length, Operand::Pointer(2))),
        Operation::Bool((
            BoolOperator::GreaterThan,
            Operand::Pointer(3),
            Operand::Terminal(TerminalType::Number(1.0)),
        )),
        Operation::Bool((BoolOperator::And, Operand::Pointer(0), Operand::Pointer(1))),
        Operation::Bool((
            BoolOperator::Or,
            Operand::Terminal(TerminalType::Number(0.0)),
            Operand::Pointer(5),
        )),
        Operation::Identity(Operand::Terminal(TerminalType::Number(1.0))),
        Operation::Identity(Operand::Terminal(TerminalType::Number(2.0))),
        Operation::Branch((
            Operand::Pointer(6),
            Operand::Pointer(7),
            Operand::Pointer(8),
        )),
    ];

    let terminal_type = operation_list[operation_list.len() - 1].evaluate(
        &operation_list,
        &mut trade_list,
        &context,
    );
    assert_eq!(terminal_type, TerminalType::Number(1.0));
}

#[test]

fn test_market_sort() {
    let context: Context = None;
    let mut trade_list = TradeList::new();

    let operation_list = vec![
        Operation::Constant((ConstantOperator::Zero, Operand::None)),
        Operation::MarketData((
            MarketDataOperator::High,
            Operand::None, //this would be the market index
            Operand::Pointer(0),
            Operand::Pointer(0),
            MarketDataInterval::Minute5,
        )),
        Operation::NumPick((NumPickOperator::Max, Operand::Pointer(0))),
        Operation::Number((NumOperator::Log, Operand::Pointer(1), Operand::None)),
        Operation::MarketSort((Operand::Pointer(2),)),
        Operation::Index((Operand::Pointer(0), Operand::Pointer(4))),
    ];
    //expect index to be 0
    let market_index = operation_list[operation_list.len() - 1].evaluate(
        &operation_list,
        &mut trade_list,
        &context,
    );
    assert_eq!(market_index, TerminalType::Number(0.0));

    let operation_list_2 = vec![
        Operation::Constant((ConstantOperator::Zero, Operand::None)),
        Operation::MarketData((
            MarketDataOperator::High,
            Operand::None, //this would be the market index from the market sort operation
            Operand::Pointer(0),
            Operand::Pointer(0),
            MarketDataInterval::Minute5,
        )),
        Operation::NumPick((NumPickOperator::Med, Operand::Pointer(1))),
        Operation::Number((
            NumOperator::Tan,
            Operand::Pointer(2),
            Operand::Terminal(TerminalType::Number(2.0)),
        )),
        Operation::Constant((ConstantOperator::BtcMarketIndex, Operand::None)),
        Operation::MarketData((
            MarketDataOperator::High,
            Operand::Pointer(4), //btc market index
            Operand::Pointer(0),
            Operand::Pointer(0),
            MarketDataInterval::Minute5,
        )),
        Operation::Number((
            NumOperator::Subtract,
            Operand::Pointer(3),
            Operand::Pointer(5),
        )),
        Operation::MarketSort((Operand::Pointer(6),)),
        // Operation::Index((Operand::Pointer(0), Operand::Pointer(4))),
    ];
    //expect index to be 3
    let market_index = operation_list_2[operation_list_2.len() - 1].evaluate(
        &operation_list_2,
        &mut trade_list,
        &context,
    );
    println!("{:?}", market_index);
}
