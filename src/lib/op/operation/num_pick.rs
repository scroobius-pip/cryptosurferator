use crate::lib::op::operand::*;
pub enum NumPickOperator {
    Average,
    Sum,
    Max,
    Min,
    Med,
    Std,
    Length,
}

///Pick operations collapase a list of types into a single type
pub type NumPickOperation = (NumPickOperator, Operand);

pub fn get_function_by_num_pick_operator(operator: &NumPickOperator) -> fn(Vec<f32>) -> f32 {
    match operator {
        NumPickOperator::Average => |list| list.iter().sum::<f32>() / list.len() as f32,
        NumPickOperator::Sum => |list| list.iter().sum::<f32>(),
        NumPickOperator::Max => |list| {
            if list.len() == 0 {
                0.0
            } else {
                list.into_iter().reduce(f32::max).unwrap()
            }
        },
        NumPickOperator::Min => |list| {
            if list.len() == 0 {
                0.0
            } else {
                list.into_iter().reduce(f32::min).unwrap()
            }
        },
        NumPickOperator::Med => |list| {
            let mut sorted = list.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let len = sorted.len();
            if len % 2 == 0 {
                (sorted.get(len / 2).unwrap_or(&0.0) + sorted.get((len / 2) + 1).unwrap_or(&0.0))
                    / 2.0
            } else {
                *sorted.get(len / 2).unwrap_or(&0.0)
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
