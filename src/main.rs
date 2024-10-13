fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct StatsCalculator;

impl StatsCalculator {
    fn calculate(_input: Vec<i32>) -> i32 {
        1
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
    fn it_should_calculate_that_the_minimum_value_in_the_range_1_2_3_4_5_is_1() {
        let input = vec![1, 2, 3, 4, 5];
        let result = StatsCalculator::calculate(input);
        assert_eq!(result, 1);
    }
}
