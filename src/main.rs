fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum StatsCalculatorErrors {
    NoValues
}

#[derive(Debug, PartialEq)]
struct StatsCalculator;

#[derive(Debug, PartialEq)]
struct StatsSummary {
    min_value: i32,
    max_value: i32,
    average: f32,
    amount_of_elements: usize
}

impl StatsCalculator {
    fn get_min_and_max_value(input: &Vec<i32>) -> (i32, i32) {
        let mut min_value: i32 = input[0];
        let mut max_value: i32 = input[0];
        for &i in input {
            if i < min_value {
                min_value = i;
            }
            if i > max_value {
                max_value = i;
            }
        }
        (min_value, max_value)
    }

    fn calculate_average(input: &Vec<i32>) -> f32 {
        let sum = input.iter().fold(0, |acc, x| acc + x);
        sum as f32 / input.len() as f32
    }

    fn calculate_amount_of_elements(input: Vec<i32>) -> usize {
        input.len()
    }

    pub fn summarize(input: Vec<i32>) -> Result<StatsSummary, StatsCalculatorErrors> {
        if input.is_empty() { return Err(StatsCalculatorErrors::NoValues); };
        let (min_value, max_value) = StatsCalculator::get_min_and_max_value(&input);
        let average = StatsCalculator::calculate_average(&input);
        let amount_of_elements = StatsCalculator::calculate_amount_of_elements(input);
        Ok(StatsSummary {
            min_value,
            max_value,
            average,
            amount_of_elements
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    fn it_should_be_defined() {
        let calculator = StatsCalculator;
        assert_eq!(calculator, StatsCalculator);
    }

    #[rstest]
    fn it_should_validate_that_values_are_given() {
        let result = StatsCalculator::summarize(vec![]);
        assert_eq!(result, Err(StatsCalculatorErrors::NoValues));
    }

    #[rstest]
    #[case(vec![1, 2, 3, 4, 5], StatsSummary {min_value: 1,max_value: 5, amount_of_elements: 5, average: 3.0 })]
    #[case(vec![3, 2, 4, 5], StatsSummary {min_value: 2,max_value: 5, amount_of_elements: 4, average: 3.5 })]
    #[case(vec![2, 4,6 ,8, 10], StatsSummary {min_value: 2,max_value: 10, amount_of_elements: 5, average: 6.0 })]
    #[case(vec![1, 2, 3, 7], StatsSummary {min_value: 1,max_value: 7, amount_of_elements: 4, average: 3.25 })]
    #[case(vec![2, 4, 21, -8, 53, 40], StatsSummary {min_value: -8, max_value: 53, amount_of_elements: 6, average: 18.666666666667 })]
    fn it_should_summarize(#[case] input: Vec<i32>, #[case] expected: StatsSummary) {
        let result = StatsCalculator::summarize(input);
        assert_eq!(result, Ok(expected));
    }
}
