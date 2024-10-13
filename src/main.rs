fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct StatsCalculator;

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    fn it_should_be_defined() {
        let calculator = StatsCalculator;
        assert_eq!(calculator, StatsCalculator);
    }
}
