type UnaryOperation<O, T> = (O, T);
type BinaryOperation<O, T> = (O, T, T);
type TernaryOperation<O, T> = (O, T, T, T);

enum Operation<U, B> {
    Unary(U),
    Binary(B),
}

enum ListTerminalType {
    Integer([i32]),
    Float([f32]),
    Index([usize]),
}

enum TerminalType {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    List(f32),
}

enum Operand {
    Pointer(usize),
    Terminal(TerminalType),
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
type BinaryBoolOperation = BinaryOperation<BinaryBoolOperator, Operand>;
//Unary boolean expression that works on a value of the same type with a unary boolean operator
type UnaryBoolOperation = UnaryOperation<UnaryBoolOperator, Operand>;
type BoolOperation = Operation<UnaryBoolOperation, BinaryBoolOperation>;
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

type MarketIndex = usize;
type MarketPrice = f32;
type MarketAmount = f32;

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

//constants mean external information

enum PortfolioMarketValue {}

//binary constant operators
enum MarketDataOperator {
    Volume,
    TradeCount,
    Open,
    High,
    Low,
    Close,
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

type UnaryNumPickOperation = UnaryOperation<NumPickOperator, ListTerminalType>;
type BinaryNumPickOperation = (NumPickOperator, ListTerminalType, usize);
type NumPickOperation = Operation<UnaryNumPickOperation, BinaryNumPickOperation>;

enum NumOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Multiply,
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
type NumOperation = Operation<UnaryNumOperation, BinaryNumOperation>;
