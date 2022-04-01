
# cryptosurferator

Cryptosurferator is a program for automatically created stock market and crypto market trading tickers using [linear genetic programming](https://en.wikipedia.org/wiki/Linear_genetic_programming). Tickers are represented as an array of instructions, with the last instruction representing the result of the  preceding ones.

Here's a basic operation, involving the addition of two numbers:

    let operation = Operation::Number((

    NumOperator::Add,
    
    Operand::Terminal(TerminalType::Number(1.0)),
    
    Operand::Terminal(TerminalType::Number(2.0)),
    
    ));

This operation is very simple and uses only the `NumOperator::Add` operator for adding two terminals, all operands have to end with a number.

There are many more operation and operator types including ones for accessing, sorting, and filtering market data.

Check `/src/lib/Operation/mod.rs` for unit tests involving more complicated examples.
