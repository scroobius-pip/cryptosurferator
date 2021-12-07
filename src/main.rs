use std::cmp::Ordering;
#[derive(Clone)]
enum TerminalType {
    Number(f32),
    NumberList(Vec<f32>),
}

impl TerminalType {
    fn to_f32(&self) -> f32 {
        match self {
            TerminalType::Number(n) => *n as f32,
            TerminalType::NumberList(n) => n.len() as f32,
        }
    }
    fn to_bool(&self) -> bool {
        match self {
            TerminalType::Number(n) => *n > 0.0,
            TerminalType::NumberList(n) => n.len() > 0,
        }
    }
    fn to_list(&self) -> Vec<f32> {
        match self {
            TerminalType::Number(n) => vec![*n as f32],
            TerminalType::NumberList(n) => n.clone(),
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

enum ConstantType {
    Number(f32),
    NumberList(Vec<f32>),
}

enum ConstantOperator {
    PortfolioValue,
    CurrentMarketPrice, //operand is the index of market
}

type ConstantOperation = (ConstantOperator, Operand);

enum Operand {
    Pointer(usize),
    Terminal(TerminalType),
    None,
}

impl Operand {
    fn evaluate(&self, operation_list: &OperationList, trade_list: &mut TradeList) -> TerminalType {
        match self {
            Operand::Pointer(pointer) => evaluate_pointer(pointer, operation_list, trade_list),
            Operand::Terminal(terminal) => terminal.clone(),
            Operand::None => TerminalType::Number(0.0),
        }
    }
}

//boolean operator that works on two values of the same type
enum BoolOperator {
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

//boolean operator that works on two boolean values
// enum BinaryBoolOperatorBool {}

//Binary boolean expression that works on two values of the same type with a binary boolean operator
type BoolOperation = (BoolOperator, Operand, Operand);
//Unary boolean expression that works on a value of the same type with a unary boolean operator

type MarketBoolOperation = (BoolOperator, Operand, Operand);

//trade operator sell, buy, or nothing

#[derive(Copy, Clone)]
enum TradeOperator {
    Buy,
    Sell,
    Nothing,
}

//amount of leverage for futures trading, x1 for normal trading
#[derive(Copy, Clone)]
enum TradeLeverage {
    X1,
    X2,
    X5,
}

type MarketIndex = Operand;
type MarketPrice = Operand;
type MarketAmount = Operand;

type TradeOperation = (
    TradeOperator,
    MarketIndex,
    MarketPrice,
    MarketAmount,
    TradeLeverage,
);

struct Trade {
    operator: TradeOperator,
    index: usize,
    price: f32,
    amount: f32,
    leverage: TradeLeverage,
}

type TradeList = Vec<Trade>;

//market data intervals
#[derive(Copy, Clone)]
enum MarketDataInterval {
    Minute1,
    Minute3,
    Minute5,
    Minute15,
    Minute30,
    Hour1,
    Hour2,
    Hour4,
    Hour6,
    Hour8,
    Hour12,
    Day1,
    Day3,
    Week1,
    Month1,
}

//binary constant operators
enum MarketDataOperator {
    Volume,
    TradeCount,
    Open,
    High,
    Low,
    Close,
    OrderBookBids,
    OrderBookAsks,
}

type MarketDataIndexStart = Operand;
type MarketDataIndexStop = Operand;
//Operation
type MarketDataOperation = (
    MarketDataOperator,
    MarketIndex,
    MarketDataIndexStart,
    MarketDataIndexStop,
    MarketDataInterval,
);

//Pick operations collapase a list of types into a single type
enum NumPickOperator {
    Average,
    Sum,
    Max,
    Min,
    Med,
    Std,
    Length,
}

type NumPickOperation = (NumPickOperator, Operand);

enum NumOperator {
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

type NumOperation = (NumOperator, Operand, Operand);

type BranchOperation = (Operand, Operand, Operand); //if else  statement

type IndexOperation = (Operand, Operand); //indexing

enum Operation {
    Branch(BranchOperation),
    Bool(BoolOperation),
    Trade(TradeOperation),
    MarketData(MarketDataOperation),
    NumPick(NumPickOperation),
    Number(NumOperation),
    Constant(ConstantOperation),
    MarketPick(MarketBoolOperation), //picks a market index to be used for market data, this is based on the ranking from an arbitrary function
    Index(IndexOperation),
}

type OperationList = Vec<Operation>;

fn main() {
    let mut operations: OperationList = vec![];
    let mut trade_list: TradeList = vec![];
    evaluate_operation(
        &operations[operations.len() - 1],
        &operations,
        &mut trade_list,
    );
}

fn evaluate_pointer(
    pointer: &usize,
    operation_list: &OperationList,
    trade_list: &mut TradeList,
) -> TerminalType {
    let operation = &operation_list[*pointer];
    evaluate_operation(operation, operation_list, trade_list)
}

fn getFunctionByNumPickOperator(operator: &NumPickOperator) -> fn(Vec<f32>) -> f32 {
    match operator {
        NumPickOperator::Average => |list| list.iter().sum::<f32>() / list.len() as f32,
        NumPickOperator::Sum => |list| list.iter().sum::<f32>(),
        NumPickOperator::Max => |list| {
            let mut max = list[0];
            for number in list.iter() {
                if number > &max {
                    max = *number;
                }
            }
            max
        },
        NumPickOperator::Min => |list| {
            let mut min = list[0];
            for number in list.iter() {
                if number < &min {
                    min = *number;
                }
            }
            min
        },
        NumPickOperator::Med => |list| {
            let mut sorted = list.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let len = sorted.len();
            if len % 2 == 0 {
                (sorted[len / 2] + sorted[(len / 2) - 1]) / 2.0
            } else {
                sorted[len / 2]
            }
        },
        NumPickOperator::Std => |list| {
            //calculate the standard deviation of list
            let mean = list.iter().sum::<f32>() / list.len() as f32;
            let mut sum = 0.0;
            for number in list.iter() {
                sum += (number - mean).powi(2);
            }
            (sum / (list.len()) as f32).sqrt()
        },
        NumPickOperator::Length => |list| list.len() as f32,
    }
}

fn evaluate_branch_terminal(
    terminal: &TerminalType,
    operand_left: &Operand,
    operand_right: &Operand,
    operation_list: &OperationList,
    trade_list: &mut TradeList,
) -> TerminalType {
    let terminal_value = terminal.to_bool();

    if terminal_value {
        operand_left.evaluate(operation_list, trade_list)
    } else {
        operand_right.evaluate(operation_list, trade_list)
    }
}

fn get_number_operator(operator: &NumOperator) -> fn(f32, f32) -> f32 {
    match operator {
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

fn evaluate_operation(
    operation: &Operation,
    operation_list: &OperationList,
    trade_list: &mut TradeList,
) -> TerminalType {
    match operation {
        Operation::Index((operand_left, operand_right)) => {
            let index = operand_left.evaluate(operation_list, trade_list).to_f32();
            let list = operand_right.evaluate(operation_list, trade_list).to_list();
            TerminalType::Number(list[index as usize])
        }

        Operation::MarketPick((operator, operand_left, operand_right)) => {
            let left_value = operand_left.evaluate(operation_list, trade_list);
            let right_value = operand_right.evaluate(operation_list, trade_list);
            let market_values: Vec<Vec<f32>> = vec![vec![3.0, 4.0, 5.0], vec![6.0, 7.0, 8.0]];
            TerminalType::NumberList(vec![0.0])
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
            let operator_function = get_number_operator(operator);
            TerminalType::Number(operator_function(left.to_f32(), right.to_f32()))
        }

        Operation::Trade((operator, market_index, market_price, market_amount, trade_leverage)) => {
            let market_index = market_index.evaluate(operation_list, trade_list).to_f32() as usize;
            let market_price = market_price.evaluate(operation_list, trade_list).to_f32();
            let market_amount = market_amount.evaluate(operation_list, trade_list).to_f32();

            trade_list.push(Trade {
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
                BoolOperator::Not => TerminalType::Number((!left_value.to_bool()) as i32 as f32),
                BoolOperator::Xor => TerminalType::Number(
                    (left_value.to_bool() ^ right_value.to_bool()) as i32 as f32,
                ),
            }
        }

        Operation::Branch((operand_operator, operand_left, operand_right)) => {
            match operand_operator {
                Operand::Pointer(pointer) => {
                    let operand_operator_value =
                        evaluate_operation(&operation_list[*pointer], operation_list, trade_list);
                    evaluate_branch_terminal(
                        &operand_operator_value,
                        operand_left,
                        operand_right,
                        operation_list,
                        trade_list,
                    )
                }
                Operand::Terminal(terminal) => evaluate_branch_terminal(
                    terminal,
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
                MarketDataOperator::TradeCount => TerminalType::NumberList(market_data.trade_count),
            }
        }
        Operation::NumPick((num_pick_operator, operand)) => {
            let operand_value = operand.evaluate(operation_list, trade_list);
            match operand_value {
                TerminalType::Number(number) => TerminalType::Number(number),
                TerminalType::NumberList(list) => {
                    let function = getFunctionByNumPickOperator(num_pick_operator);
                    TerminalType::Number(function(list))
                }
            }
        }
    }
}

fn get_market_data(
    market_index: usize,
    data_index_start: usize,
    data_index_stop: usize,
    market_interval: MarketDataInterval,
) -> MarketData {
    let market_data = MarketData {
        open: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        high: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        low: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        close: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        volume: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        asks: vec![],
        bids: vec![],
        trade_count: vec![],
    };
    market_data
}

struct MarketData {
    open: Vec<f32>,
    high: Vec<f32>,
    low: Vec<f32>,
    close: Vec<f32>,
    volume: Vec<f32>,
    trade_count: Vec<f32>,
    bids: Vec<f32>,
    asks: Vec<f32>,
}
