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
}

impl StatsCalculator {
    fn get_min_value(input: Vec<i32>) -> i32 {
        let mut min_value: i32 = input[0];
        for &i in &input {
            if i < min_value {
                min_value = i;
            }
        }
        min_value
    }

    pub fn calculate(input: Vec<i32>) -> Result<i32, StatsCalculatorErrors> {
        if input.is_empty() { return Err(StatsCalculatorErrors::NoValues); };
        let min_value = StatsCalculator::get_min_value(input);
        Ok(min_value)
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
        let result = StatsCalculator::calculate(vec![]);
        assert_eq!(result, Err(StatsCalculatorErrors::NoValues));
    }

    #[rstest]
    #[case(vec![1, 2, 3, 4, 5], 1)]
    #[case(vec![3, 2, 4, 5], 2)]
    fn it_should_calculate_the_minimum_value(#[case] input: Vec<i32>, #[case] expected: i32) {
        let result = StatsCalculator::calculate(input);
        assert_eq!(result, Ok(expected));
    }
}
