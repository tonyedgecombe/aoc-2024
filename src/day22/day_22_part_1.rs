use crate::day22::day_22_standard::*;

fn sum_of_two_thousandth(initial_secrets: &[i64]) -> i64 {
    initial_secrets.iter().map(|secret| two_thousandth(*secret)).sum()
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_sum() {
        let lines = parse_data(EXAMPLE_DATA);
        let sum = sum_of_two_thousandth(&lines);

        assert_eq!(sum, 37327623);
    }

    #[test]
    fn test_day_22_part_1() {
        let secrets = parse_data(include_str!("../data/day_22_input.txt"));
        let result = sum_of_two_thousandth(&secrets);

        assert_eq!(result, 17163502021);
    }
}