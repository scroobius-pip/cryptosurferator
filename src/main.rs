enum TerminalType {
    Number(f32),
    NumberList(Box<[f32]>),
}

enum ConstantType {
    Number(f32),
    NumberList(Box<[f32]>),
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
    And,
    Or,
    Xor,
    Identity,
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

enum ConstantOperator {
    PortfolioMarketValue, //gives the amount of units of a particular currency in a portfolio
    CurrentMarketPrice,
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
    Index,
    Length,
}

type NumPickOperation = (NumPickOperator, Operand, Operand);

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

enum Operation {
    Branch(BranchOperation),
    Bool(BoolOperation),
    Trade(TradeOperation),
    MarketData(MarketDataOperation),
    NumPick(NumPickOperation),
    Number(NumOperation),
    Constant(ConstantOperation),
    MarketPick(MarketBoolOperation), //picks a market index to be used for market data, this is based on the ranking from an arbitrary function
}

type OperationList = Vec<Operation>;
fn main() {
    let mut operations: OperationList = vec![
        Operation::MarketData((
            MarketDataOperator::Volume,
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(1.0)),
            MarketDataInterval::Minute1,
        )),
        Operation::NumPick((NumPickOperator::Average, Operand::Pointer(0), Operand::None)),
        Operation::Number((
            NumOperator::Cos,
            Operand::Terminal(TerminalType::Number(0.0)),
            Operand::None,
        )),
        Operation::NumPick((NumPickOperator::Index, Operand::Pointer(2), Operand::None)),
        Operation::Number((
            NumOperator::Add,
            Operand::Pointer(2),
            Operand::Terminal(TerminalType::Number(0.0)),
        )),
        Operation::Trade((
            TradeOperator::Buy,
            Operand::Terminal(TerminalType::Number(0.0)),
            Operand::Pointer(3),
            Operand::Terminal(TerminalType::Number(1.1)),
            TradeLeverage::X1,
        )),
        Operation::Bool((
            BoolOperator::GreaterThan,
            Operand::Terminal(TerminalType::Number(0.2)),
            Operand::Pointer(1),
        )),
        Operation::Constant((ConstantOperator::PortfolioValue, Operand::None)),
        Operation::Bool((
            BoolOperator::GreaterThan,
            Operand::Terminal(TerminalType::Number(0.2)),
            Operand::Pointer(1),
        )),
        Operation::Branch((
            Operand::Pointer(3),
            Operand::Terminal(TerminalType::Number(1.0)),
            Operand::Terminal(TerminalType::Number(0.0)),
        )),
        Operation::Trade((
            TradeOperator::Sell,
            Operand::Terminal(TerminalType::Number(0.0)),
            Operand::Pointer(3),
            Operand::Terminal(TerminalType::Number(1.1)),
            TradeLeverage::X2,
        )),
        Operation::Branch((
            Operand::Pointer(6),
            Operand::Pointer(5),
            Operand::Pointer(2),
        )),
    ];
}


fn evaluate_operation(operation: Operation, state: &mut State) -> f64 {
    match operation {
        Operation::Branch((operator,operand_left,operand_right)) => {
            let condition = evaluate_operation(operation.0, state);
            if condition > 0.0 {
                evaluate_operation(operation.1, state)
            } else {
                evaluate_operation(operation.2, state)
            }
        }
        Operation::Bool(operation) => {
            let left = evaluate_operation(operation.1, state);
            let right = evaluate_operation(operation.2, state);
            match operation.0 {
                BoolOperator::Equal => left == right,
                BoolOperator::NotEqual => left != right,
                BoolOperator::GreaterThan => left > right,
                BoolOperator::GreaterThanOrEqual => left >= right,
                BoolOperator::LessThan => left < right,
                BoolOperator::And => left > 0.0 && right > 0.0,
                BoolOperator::Or => left > 0.0 || right > 0.0,
                BoolOperator::Xor => left > 0.0 ^ right > 0.0,
                BoolOperator::Identity => left == right,
                BoolOperator::Not => left == 0.0,
            }
        }
        Operation::Trade(operation) => {
            let market_index = evaluate_operation(operation.1, state);
            let price = evaluate_operation(operation.2, state);
            let amount = evaluate_operation(operation.3, state);
            let leverage = match operation.4 {
                TradeLeverage::X1 => 1.0,
                TradeLeverage::X2 => 2.0,
                TradeLeverage::X3 => 3.0,
                TradeLeverage::X4 => 4.0,
                TradeLeverage::X5 => 5.0,
            };
            match operation.0 {
                TradeOperator::Buy => state.buy(market_index, price, amount, leverage),
                TradeOperator::Sell => state.sell(market_index, price, amount, leverage),
            }
        }
        Operation::MarketData(operation) => {
            let market_index = evaluate_operation(operation.1, state);
            let start = evaluate_operation(operation.2, state);
            let stop = evaluate_operation(operation.3, state);
            let interval = match operation.4 {
                MarketDataInterval::Minute1 => 1,
                MarketDataInterval::Minute5 => 5,
                MarketDataInterval::Minute15 => 15,
                MarketDataInterval::Minute30 => 30,
                MarketDataInterval::Hour1 => 60,
                MarketDataInterval::Hour4 => 240,
                MarketDataInterval::Day1 => 1440,
                MarketDataInterval::Week1 => 10080,
                MarketDataInterval::Month1 => 43800,
            };
            match operation.0 {
                MarketDataOperator::Volume => state.market_data.volume(market_index, start, stop, interval),
                MarketDataOperator::TradeCount => state.market_data.trade_count(market_index, start, stop, interval),
                MarketDataOperator::Open => state.market_data.open(market_index, start, stop, interval),
                MarketDataOperator::High => state.market_data.high(market_index, start, stop, interval),
                MarketDataOperator::Low => state.market_data.low(market_index, start, stop, interval),
                MarketDataOperator::Close => state.market_data.close(market_index, start, stop, interval),
                MarketDataOperator::OrderBookBids => state.market_data.order_book_bids(market_index, start, stop, interval),
                MarketDataOperator::OrderBookAsks => state.market_data.order_book_asks(market_index, start, stop, interval),
            }
        }
        Operation::Num(operation) => {
            let left = evaluate_operation(operation.1, state);
            let right = evaluate_operation(operation.2, state);
            match operation.0 {
                NumOperator::Add => left + right,
                NumOperator::Subtract => left - right,
                NumOperator::Multiply => left * right,
                NumOperator::Divide => left / right,
                NumOperator::Modulo => left % right,
                NumOperator::Power => left.powf(right),
                NumOperator::Abs => left.abs(),
                NumOperator::Cos => left.cos(),
                NumOperator::Sin => left.sin(),
                NumOperator::Tan => left.tan(),
                NumOperator::Cot => left.tan().recip(),
                NumOperator::Sqrt => left.sqrt(),
                NumOperator::Log => left.ln(),
                NumOperator::Log10 => left.log10(),
                NumOperator::Exp => left.exp(),
                NumOperator::Floor => left.floor(),
                NumOperator::Ceil => left.ceil(),
                NumOperator::Round => left.round(),
                NumOperator::Truncate => left.trunc(),
                NumOperator::Min => left.min(right),
                NumOperator::Max => left.max(right),
                NumOperator::Average => (left + right) / 2.0,
                NumOperator::Index => left.powf(right),
            }
        }
        Operation::Pointer(operation) => {
            let index = evaluate_operation(operation, state);
            state.stack[index as usize]
        }
        Operation::Terminal(operation) => {
            match operation {
                TerminalType::Number(number) => number,
            
            }
        }
    }
}

