#[derive(Copy, Clone)]
enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulo,
    Print,
}

fn Add(a: f32, b: f32) -> f32 {
    a + b
}

fn Subtract(a: f32, b: f32) -> f32 {
    a - b
}

fn Divide(a: f32, b: f32) -> f32 {
    a / b
}

fn Multiply(a: f32, b: f32) -> f32 {
    a * b
}

fn Modulo(a: f32, b: f32) -> f32 {
    a % b
}

fn Print(a: f32, _: f32) -> f32 {
    println!("{}", a);
    a
}

fn getFunctionByOperator(operator: &Operator) -> fn(f32, f32) -> f32 {
    match operator {
        Operator::Add => Add,
        Operator::Subtract => Subtract,
        Operator::Divide => Divide,
        Operator::Multiply => Multiply,
        Operator::Modulo => Modulo,
        Operator::Print => Print,
    }
}

type UnaryOperation = (Operator, Operand);
type BinaryOperation = (Operator, Operand, Operand);

#[derive(Copy, Clone)]
enum Operation {
    Unary(UnaryOperation),
    Binary(BinaryOperation),
}

type OperationList = Vec<Operation>;

enum TerminalType {
    Number(f32),
    Boolean(bool),
}
#[derive(Copy, Clone)]
enum Operand {
    // Pointer is an index that refers to an operation in an operation list
    Pointer(usize),
    Terminal(f32),
}

fn evaluateOperation(operation: &Operation, operation_list: &OperationList) -> f32 {
    match operation {
        Operation::Unary(operation) => {
            let operand = operation.1;
            let operator = &operation.0;
            match operand {
                Operand::Pointer(operation) => {
                    let operation = &operation_list[operation];
                    getFunctionByOperator(operator)(
                        evaluateOperation(operation, operation_list),
                        0.0,
                    )
                }
                Operand::Terminal(terminal) => getFunctionByOperator(operator)(terminal, 0.0),
            }
        }
        Operation::Binary(operation) => {
            let operator = &operation.0;
            let left_operand = operation.1;
            let right_operand = operation.2;
            match left_operand {
                Operand::Pointer(operation) => {
                    let operation = &operation_list[operation];
                    let left_operand = evaluateOperation(operation, operation_list);
                    match right_operand {
                        Operand::Pointer(operation) => {
                            let operation = &operation_list[operation];
                            let right_operand = evaluateOperation(operation, operation_list);
                            getFunctionByOperator(&operator)(left_operand, right_operand)
                        }
                        Operand::Terminal(terminal) => {
                            getFunctionByOperator(&operator)(left_operand, terminal)
                        }
                    }
                }
                Operand::Terminal(terminal) => match right_operand {
                    Operand::Pointer(operation) => {
                        let operation = &operation_list[operation];
                        let right_operand = evaluateOperation(operation, operation_list);
                        getFunctionByOperator(&operator)(terminal, right_operand)
                    }
                    Operand::Terminal(right_terminal) => {
                        getFunctionByOperator(&operator)(terminal, right_terminal)
                    }
                },
            }
        }
    }
}

fn main() {
    // let print_operation: Operation =
    let operation_list: OperationList = vec![
        Operation::Binary((
            Operator::Multiply,
            Operand::Terminal(10 as f32),
            Operand::Terminal(10 as f32),
        )),
        Operation::Binary((
            Operator::Subtract,
            Operand::Pointer(0),
            Operand::Terminal(2 as f32),
        )),
        Operation::Binary((Operator::Divide, Operand::Pointer(1), Operand::Pointer(0))),
        Operation::Unary((Operator::Print, Operand::Pointer(2))),
    ];
    for operation in &operation_list {
        evaluateOperation(&operation, &operation_list);
        // println!("{:?}", operation);
    }
}
