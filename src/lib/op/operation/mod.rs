pub mod boolean;
pub mod branch;
pub mod constant;
pub mod index;
pub mod market_data;
pub mod market_sort;
pub mod num_pick;
pub mod number;
pub mod trade;

use crate::lib::op::environment::Env;
use crate::lib::op::operand::*;
use crate::lib::op::terminal_type::*;
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
    ///Pick operations collapase a list of types into a single type
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
        env: &impl Env,
    ) -> TerminalType {
        match self {
            Operation::MarketSort((operand,)) => {
                let mut market_index_list = env.get_market_index_list();
                market_index_list.sort_by(|market_index_a, market_index_b| {
                    let value_a = operand
                        .evaluate(
                            operation_list,
                            trade_list,
                            &Some(TerminalType::Number(*market_index_a)),
                            env,
                        )
                        .to_f32(); //call with market_index_a
                    let value_b = operand
                        .evaluate(
                            operation_list,
                            trade_list,
                            &Some(TerminalType::Number(*market_index_b)),
                            env,
                        )
                        .to_f32(); //call with market_index_b
                    value_a.partial_cmp(&value_b).unwrap()
                });
                TerminalType::NumberList(market_index_list)
            }
            Operation::Identity(operand) => {
                operand.evaluate(operation_list, trade_list, context, env)
            }
            Operation::Index((operator, operand_right)) => {
                let list = operand_right
                    .evaluate(operation_list, trade_list, context, env)
                    .to_list();

                let index = match operator {
                    IndexOperator::First => 0,
                    IndexOperator::Last => list.len() - 1,
                    IndexOperator::Operand(operand) => {
                        let index = operand
                            .evaluate(operation_list, trade_list, context, env)
                            .to_usize();
                        index.max(0).min(list.len() - 1)
                    }
                };

                TerminalType::Number(list[index])
            }
            Operation::Constant((operator, operand)) => match operator {
                ConstantOperator::MarketPrice => {
                    let market_index = operand.evaluate(operation_list, trade_list, context, env);
                    TerminalType::Number(env.get_market_price(market_index.to_usize()))
                }
                ConstantOperator::PortfolioValue => {
                    let overall_portfolio_value = env.get_overall_portfolio_value();
                    TerminalType::Number(overall_portfolio_value)
                }
                ConstantOperator::SelectedMarketPortfolioValue => {
                    let market_index = operand.evaluate(operation_list, trade_list, context, env);
                    TerminalType::Number(env.get_market_portfolio_value(market_index.to_usize()))
                }

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
            },
            Operation::Number((operator, operand_left, operand_right)) => {
                let left = operand_left.evaluate(operation_list, trade_list, context, env);
                let right = operand_right.evaluate(operation_list, trade_list, context, env);
                TerminalType::Number(operator.func()(left.to_f32(), right.to_f32()))
            }

            Operation::Trade((operator, market_index, market_price, market_amount)) => {
                let market_index = market_index
                    .evaluate(operation_list, trade_list, context, env)
                    .to_usize();
                let market_price = market_price
                    .evaluate(operation_list, trade_list, context, env)
                    .to_f32();
                let market_amount = market_amount
                    .evaluate(operation_list, trade_list, context, env)
                    .to_f32();
                trade_list.push(trade::Trade {
                    operator: *operator,
                    index: market_index,
                    price: market_price,
                    amount: market_amount,
                });
                TerminalType::Number(1.0)
            }
            Operation::Bool((operator, operand_left, operand_right)) => {
                let left_value = operand_left.evaluate(operation_list, trade_list, context, env);
                let right_value = operand_right.evaluate(operation_list, trade_list, context, env);
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
                            .evaluate(operation_list, trade_list, context, env);

                        operand_operator_value.evaluate_branch_terminal(
                            operand_left,
                            operand_right,
                            operation_list,
                            trade_list,
                            context,
                            env,
                        )
                    }
                    Operand::Terminal(terminal) => terminal.evaluate_branch_terminal(
                        operand_left,
                        operand_right,
                        operation_list,
                        trade_list,
                        context,
                        env,
                    ),
                    Operand::None => TerminalType::Number(0.0),
                }
            }
            Operation::MarketData((
                market_data_operator,
                market_index_operand,
                timestamp_start_operand,
                timestamp_duration_operand,
            )) => {
                //if context exists use it instead of market_index_operand
                // market_index_operand will only be evaluated normally if there is no context and itself is Operand::None
                // if there is a context, it will be used if it market_index_operand == Operand::None
                let market_index_value = match (market_index_operand, context) {
                    (Operand::None, Context::Some(context_terminal_type)) => {
                        context_terminal_type.clone()
                    }
                    _ => market_index_operand.evaluate(operation_list, trade_list, context, env),
                };

                let timestamp_start_value =
                    timestamp_start_operand.evaluate(operation_list, trade_list, context, env);
                let timestamp_duration_value =
                    timestamp_duration_operand.evaluate(operation_list, trade_list, context, env);

                let market_data = env.get_market_data(
                    market_index_value.to_usize(),
                    timestamp_start_value.to_f32(),
                    timestamp_duration_value.to_f32(),
                );
                match market_data_operator {
                    MarketDataOperator::Open => TerminalType::NumberList(market_data.open),
                    MarketDataOperator::High => TerminalType::NumberList(market_data.high),
                    MarketDataOperator::Low => TerminalType::NumberList(market_data.low),
                    MarketDataOperator::Close => TerminalType::NumberList(market_data.close),
                    MarketDataOperator::Volume => TerminalType::NumberList(market_data.volume),
                    MarketDataOperator::TradeCount => {
                        TerminalType::NumberList(market_data.trade_count)
                    }
                }
            }
            Operation::NumPick((num_pick_operator, operand)) => {
                let operand_value = operand
                    .evaluate(operation_list, trade_list, context, env)
                    .to_list();
                let function = get_function_by_num_pick_operator(num_pick_operator);

                TerminalType::Number(function(operand_value))
            }
        }
    }

    // pub fn mutate(&self) -> Self {
    //     match self {
    //         Operation::Branch(operation) => {
    //             let mut new_operand_left = operation.0.1.clone();
    //             let mut new_operand_right = operation.0.2.clone();
    //             new_operand_left.mutate();
    //             new_operand_right.mutate();
    //             Operation::Branch((operation.0.0, new_operand_left, new_operand_right))
            
    //         }
    //         _ => panic!("Not implemented"),
    //     }
    // }
}

//tests
#[cfg(test)]

mod tests {
    use crate::lib::op::{environment::Env, operand::*, operation::*, terminal_type::*};

    struct DefaultEnv {}
    impl Env for DefaultEnv {}

    #[test]
    fn test_evaluate_operation() {
        let DefaultEnv = DefaultEnv {};
        let context = None;
        let mut operation_list = OperationList::new();
        let mut trade_list = TradeList::new();
        let operation =
            Operation::Number((NumOperator::Add, Operand::Pointer(0), Operand::Pointer(1)));
        operation_list.push(Operation::Constant((ConstantOperator::One, Operand::None)));
        operation_list.push(Operation::Constant((ConstantOperator::Two, Operand::None)));
        let terminal_type =
            operation.evaluate(&operation_list, &mut trade_list, &context, &DefaultEnv);
        assert_eq!(terminal_type, TerminalType::Number(3.0));
    }

    #[test]
    fn test_evaluate_operation_number_constants() {
        let DefaultEnv = DefaultEnv {};
        let context = None;
        let operation_list = OperationList::new();
        let mut trade_list = TradeList::new();
        let operation = Operation::Number((
            NumOperator::Add,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(2.0)),
        ));
        let terminal_type =
            operation.evaluate(&operation_list, &mut trade_list, &context, &DefaultEnv);
        assert_eq!(terminal_type, TerminalType::Number(3.0));
    }

    #[test]
    fn test_bool_operation() {
        let DefaultEnv = DefaultEnv {};
        let context = None;
        let operation_list = OperationList::new();
        let mut trade_list = TradeList::new();
        let operation = Operation::Bool((
            BoolOperator::Equal,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(2.0)),
        ));
        let terminal_type =
            operation.evaluate(&operation_list, &mut trade_list, &context, &DefaultEnv);
        assert_eq!(terminal_type, TerminalType::Number(0.0));
    }
    #[test]
    fn test_branch_operation() {
        let DefaultEnv = DefaultEnv {};
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
        let terminal_type =
            operation.evaluate(&operation_list, &mut trade_list, &context, &DefaultEnv);
        assert_eq!(terminal_type, TerminalType::Number(2.0));

        let terminal_type =
            operation.evaluate(&operation_list_2, &mut trade_list, &context, &DefaultEnv);
        assert_eq!(terminal_type, TerminalType::Number(1.0));
    }

    #[test]
    fn test_num_pick_operation() {
        let default_env = DefaultEnv {};
        let context = None;
        let operation_list = OperationList::new();
        let mut trade_list = TradeList::new();
        let operation = Operation::NumPick((
            NumPickOperator::Max,
            Operand::Terminal(TerminalType::NumberList(vec![1.0, 2.0, 3.0])),
        ));
        let terminal_type =
            operation.evaluate(&operation_list, &mut trade_list, &context, &default_env);
        assert_eq!(terminal_type, TerminalType::Number(3.0));
    }

    #[test]
    fn test_num_pic_with_branch_operation() {
        let DefaultEnv = DefaultEnv {};
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
            &DefaultEnv,
        );
        assert_eq!(terminal_type, TerminalType::Number(1.0));
    }
    #[test]
    fn test_trade_operation() {
        let DefaultEnv = DefaultEnv {};
        let context = None;
        let mut trade_list = TradeList::new();
        let operation_list = vec![
            Operation::Trade((
                TradeOperator::Buy,
                Operand::Terminal(TerminalType::Number(1.0)),
                Operand::Terminal(TerminalType::Number(2.0)),
                Operand::Terminal(TerminalType::Number(3.0)),
            )),
            Operation::Trade((
                TradeOperator::Sell,
                Operand::Terminal(TerminalType::Number(1.0)),
                Operand::Terminal(TerminalType::Number(2.0)),
                Operand::Terminal(TerminalType::Number(3.0)),
            )),
            Operation::MarketData((
                MarketDataOperator::High,
                Operand::Terminal(TerminalType::Number(1.0)),
                Operand::Terminal(TerminalType::Number(1.0)),
                Operand::Terminal(TerminalType::Number(1.0)),
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

        operation_list[operation_list.len() - 1].evaluate(
            &operation_list,
            &mut trade_list,
            &context,
            &DefaultEnv,
        );

        assert_eq!(trade_list.len(), 1);
        assert_eq!(trade_list[0].operator, TradeOperator::Sell);
    }
    #[test]
    fn test_multiple_bool_operation() {
        let DefaultEnv = DefaultEnv {};
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
            &DefaultEnv,
        );
        assert_eq!(terminal_type, TerminalType::Number(1.0));
    }

    #[test]

    fn test_market_sort() {
        struct D {}
        impl Env for D {
            fn get_market_index_list(&self) -> Vec<f32> {
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
            }

            fn get_market_data(
                &self,
                market_index: usize,
                timestamp_start: f32,
                duration: f32,
            ) -> MarketData {
                let market_data = MarketData {
                    close: vec![],
                    high: vec![if market_index == 6 { 1000.0 } else { 0.0 }],
                    low: vec![],
                    open: vec![],
                    trade_count: vec![],
                    volume: vec![],
                };

                market_data
            }

            //
        }
        let context: Context = None;
        let mut trade_list = TradeList::new();

        let operation_list = vec![
            Operation::Constant((ConstantOperator::Zero, Operand::None)),
            Operation::MarketData((
                MarketDataOperator::High,
                Operand::None, //this would be the market index
                Operand::Pointer(0),
                Operand::Pointer(0),
            )),
            Operation::NumPick((NumPickOperator::Max, Operand::Pointer(1))),
            Operation::MarketSort((Operand::Pointer(2),)),
            Operation::Index((IndexOperator::Last, Operand::Pointer(3))),
        ];
        //expect index to be 6.0
        let market_index = operation_list[operation_list.len() - 1].evaluate(
            &operation_list,
            &mut trade_list,
            &context,
            &D {},
        );
        assert_eq!(market_index, TerminalType::Number(6.0));
    }
}
