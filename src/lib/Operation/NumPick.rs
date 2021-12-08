use crate::lib::Operand::*;
//Pick operations collapase a list of types into a single type
pub enum NumPickOperator {
    Average,
    Sum,
    Max,
    Min,
    Med,
    Std,
    Length,
}

pub type NumPickOperation = (NumPickOperator, Operand);

pub fn get_function_by_num_pick_operator(operator: &NumPickOperator) -> fn(Vec<f32>) -> f32 {
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
                (sorted[len / 2] + sorted[(len / 2) + 1]) / 2.0
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
