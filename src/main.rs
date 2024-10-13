fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum StatsCalculatorErrors {
    NoValues
}

#[derive(Debug, PartialEq)]
struct StatsCalculator;

impl StatsCalculator {
    fn calculate(input: Vec<i32>) -> Result<i32, StatsCalculatorErrors> {
        if input.is_empty() {
            return Err(StatsCalculatorErrors::NoValues);
        }

        let mut min_value: i32 = input[0];

        for &i in &input {
            if i < min_value {
                min_value = i;
            }
        }
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
    fn it_should_calculate_that_the_minimum_value_in_the_range_1_2_3_4_5_is_1() {
        let input = vec![1, 2, 3, 4, 5];
        let result = StatsCalculator::calculate(input);
        assert_eq!(result, Ok(1));
    }

    #[rstest]
    fn it_should_calculate_that_the_minimum_value_in_the_range_3_2_4_5_is_2() {
        let input = vec![3, 2, 4, 5];
        let result = StatsCalculator::calculate(input);
        assert_eq!(result, Ok(2));
    }
}
