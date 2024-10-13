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
}

impl StatsCalculator {
    fn get_min_value(input: &Vec<i32>) -> i32 {
        let mut min_value: i32 = input[0];
        for &i in input {
            if i < min_value {
                min_value = i;
            }
        }
        min_value
    }
    fn get_max_value(input: Vec<i32>) -> i32 {
        let mut max_value: i32 = input[0];
        for i in input {
            if i > max_value {
                max_value = i;
            }
        }
        max_value
    }

    pub fn summarize(input: Vec<i32>) -> Result<StatsSummary, StatsCalculatorErrors> {
        if input.is_empty() { return Err(StatsCalculatorErrors::NoValues); };
        let min_value = StatsCalculator::get_min_value(&input);
        let max_value = StatsCalculator::get_max_value(input);
        Ok(StatsSummary {
            min_value,
            max_value
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
    #[case(vec![1, 2, 3, 4, 5], StatsSummary {min_value: 1,max_value: 5 })]
    #[case(vec![3, 2, 4, 5], StatsSummary {min_value: 2,max_value: 5 })]
    fn it_should_calculate_the_minimum_value(#[case] input: Vec<i32>, #[case] expected: StatsSummary) {
        let result = StatsCalculator::summarize(input);
        assert_eq!(result, Ok(expected));
    }
}
