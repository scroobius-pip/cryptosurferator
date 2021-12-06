use std::cmp::Ordering;

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
    CurrentMarketPrice,
}

type ConstantOperation = (ConstantOperator, Operand);

enum Operand {
    Pointer(usize),
    Terminal(TerminalType),
    None,
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
enum TradeOperator {
    Buy,
    Sell,
    Nothing,
}

//amount of leverage for futures trading, x1 for normal trading
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

//market data intervals
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
    Sum,
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
}

fn evaluate_pointer(pointer: &usize, operation_list: &OperationList) -> TerminalType {
    let operation = &operation_list[*pointer];
    evaluate_operation(operation, operation_list)
}

fn evaluate_terminal(terminal: &TerminalType) -> TerminalType {
    match terminal {
        TerminalType::Number(number) => TerminalType::Number(*number),
        TerminalType::NumberList(list) => TerminalType::NumberList(*list),
    }
}

fn evaluate_operand(operand: &Operand, operation_list: &OperationList) -> TerminalType {
    match operand {
        Operand::Pointer(pointer) => evaluate_pointer(pointer, operation_list),
        Operand::Terminal(terminal) => *terminal,
        Operand::None => TerminalType::Number(0.0),
    }
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
) -> TerminalType {
    let terminal_value = terminal.to_bool();

    if terminal_value {
        evaluate_operand(operand_left, operation_list)
    } else {
        evaluate_operand(operand_right, operation_list)
    }
}

fn evaluate_operation(operation: &Operation, operation_list: &OperationList) -> TerminalType {
    match operation {

        

        Operation::Trade((operator, market_index, market_price, market_amount, trade_leverage)) => {
            let market_index = evaluate_operand(market_index, operation_list).to_f32() as usize;
            let market_price = evaluate_operand(market_price, operation_list).to_f32();
            let market_amount = evaluate_operand(market_amount, operation_list).to_f32();
            // let trade_leverage = evaluate_operand(trade_leverage, operation_list);


            TerminalType::Number(0.0)
        }
        Operation::Bool((operator, operand_left, operand_right)) => {
            let left_value = evaluate_operand(operand_left, operation_list);
            let right_value = evaluate_operand(operand_right, operation_list);

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
                        evaluate_operation(&operation_list[*pointer], operation_list);
                    evaluate_branch_terminal(
                        &operand_operator_value,
                        operand_left,
                        operand_right,
                        operation_list,
                    )
                }
                Operand::Terminal(terminal) => {
                    evaluate_branch_terminal(terminal, operand_left, operand_right, operation_list)
                }
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
            let market_index_value = evaluate_operand(market_index_operand, operation_list);
            let data_index_start_value = evaluate_operand(data_index_start_operand, operation_list);
            let data_index_stop_value = evaluate_operand(data_index_stop_operand, operation_list);

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
            }
        }
        Operation::NumPick((num_pick_operator, operand)) => {
            let operand_value = evaluate_operand(operand, operation_list);
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
