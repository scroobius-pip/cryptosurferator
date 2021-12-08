mod lib;

use lib::{Operation::Trade::TradeList, Operation::*};

fn main() {
    let operations: OperationList = vec![];
    let mut trade_list: TradeList = vec![];

    operations[operations.len() - 1].evaluate(&operations, &mut trade_list);

    //print all items in  TradeList vector
    for trade in trade_list {
        println!("{}", trade);
    }
}
