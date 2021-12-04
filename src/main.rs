type UnaryOperation<O, T> = (O, T);
type BinaryOperation<O, T> = (O, T, T);
type TernaryOperation<O, T> = (O, T, T, T);

enum GenericOperation<U, B> {
    Unary(U),
    Binary(B),
}

enum ListTerminalType {
    Integer(Box<[i32]>),
    Float(Box<[f32]>),
    Index(Box<[usize]>),
}

enum TerminalType {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Index(usize),
    Parameter(Parameter),
}

enum Parameter {
    Market(MarketParamConstant),
}

enum Operand {
    Pointer(usize),
    Terminal(TerminalType),
    Parameter(Parameter),
}

//boolean operator that works on two values of the same type
enum BinaryBoolOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    And,
    Or,
    Xor,
}

enum UnaryBoolOperator {
    Not, //works on any type, negative numbers are false and positive numbers are true
}

//boolean operator that works on two boolean values
// enum BinaryBoolOperatorBool {}

//Binary boolean expression that works on two values of the same type with a binary boolean operator
type BinaryBoolOperation = (BinaryBoolOperator, Operand, Operand);
//Unary boolean expression that works on a value of the same type with a unary boolean operator
type UnaryBoolOperation = (UnaryBoolOperator, Operand);
type BoolOperation = GenericOperation<UnaryBoolOperation, BinaryBoolOperation>;
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

enum MarketParamConstant {
    MarketParamVolume,
    MarketParamTradeCount,
    MarketParamOpen,
    MarketParamHigh,
    MarketParamLow,
    MarketParamClose,
    MarketParamOrderBookBids,
    MarketParamOrderBookAsks,
}

type MarketDataTimeStart = i32;
type MarketDataTimeEnd = i32;

type MarketDataOperation = (
    MarketDataOperator,
    MarketDataTimeStart,
    MarketDataTimeEnd,
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
}

type UnaryNumPickOperation = UnaryOperation<NumPickOperator, Operand>;
type BinaryNumPickOperation = (NumPickOperator, Operand, Operand);
type NumPickOperation = GenericOperation<UnaryNumPickOperation, BinaryNumPickOperation>;

enum NumOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Min,
    Max,
    Sum,
    Average,
    Cos,
    Sin,
    Tan,
    Pow,
    Log,
}

type UnaryNumOperation = UnaryOperation<NumOperator, Operand>;
type BinaryNumOperation = BinaryOperation<NumOperator, Operand>;
type NumOperation = GenericOperation<UnaryNumOperation, BinaryNumOperation>;
// enum NumOperation {
//     Unary(UnaryNumOperation),
//     Binary(BinaryNumOperation),
// }

enum ConstantListOperator {}

enum ConstantOperator {
    PortfolioMarketValue, //gives the amount of units of a particular currency in a portfolio
    CurrentMarketPrice,
}

type BranchCondition = Operand;
type TrueBranch = Operand;
type FalseBranch = Operand;

type BranchOperation = (BranchCondition, TrueBranch, FalseBranch); //if else  statement

enum Operation {
    Branch(BranchOperation),
    Bool(BoolOperation),
    Trade(TradeOperation),
    MarketData(MarketDataOperation),
    NumPick(NumPickOperation),
    Number(NumOperation),
    MarketPick(BoolOperation), //picks a market index to be used for market data, this is based on the ranking from an arbitrary function
}

type OperationList = Vec<Operation>;
fn main() {
    let mut operations: OperationList = vec![
        Operation::MarketData((
            MarketDataOperator::Volume,
            0,
            0,
            MarketDataInterval::Minute1,
        )),
        Operation::NumPick(NumPickOperation::Unary((
            NumPickOperator::Average,
            Operand::Pointer(0),
        ))),
        Operation::NumPick(NumPickOperation::Binary((
            NumPickOperator::Index,
            Operand::Pointer(0),
            Operand::Terminal(TerminalType::Index(0)),
        ))),
        Operation::Number(NumOperation::Unary((NumOperator::Cos, Operand::Pointer(1)))),
        Operation::Trade((
            TradeOperator::Buy,
            Operand::Terminal(TerminalType::Index(0)),
            Operand::Pointer(3),
            Operand::Terminal(TerminalType::Float(1.1)),
            TradeLeverage::X1,
        )),
        Operation::Trade((
            TradeOperator::Sell,
            Operand::Terminal(TerminalType::Index(0)),
            Operand::Pointer(3),
            Operand::Terminal(TerminalType::Float(1.1)),
            TradeLeverage::X2,
        )),
        Operation::Bool(BoolOperation::Binary((
            BinaryBoolOperator::LessThan,
            Operand::Terminal(TerminalType::Integer(100)),
            Operand::Pointer(1),
        ))),
        Operation::NumPick(NumPickOperation::Unary((
            NumPickOperator::Average,
            Operand::Parameter(Parameter::Market(MarketParamConstant::MarketParamHigh)),
        ))),
        Operation::MarketPick(BoolOperation::Binary((
            BinaryBoolOperator::GreaterThan,
            Operand::Pointer(8),
            Operand::Pointer(8),
        ))),
        Operation::Branch((
            Operand::Pointer(6),
            Operand::Pointer(5),
            Operand::Pointer(2),
        )),
    ];
}

fn evaluate_operation(operation: &Operation, operation_list: &OperationList) {
    match operation {
        Operation::MarketPick(boolOperation) => match boolOperation {
            GenericOperation::Binary((operator, operand1, operand2)) => match operand1 {
                Operand::Parameter(parameter) => match parameter {
                    Parameter::Market(market_param_constant) => {
                        let constant = vec![];
                        
                    }
                },
                Operand::Pointer(operation) => {}
                Operand::Terminal(terminal) => {}
            },
            GenericOperation::Unary((operator, operand1)) => {}
        },
        _ => {}
    }
}
