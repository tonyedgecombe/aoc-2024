use crate::day24::day_24_common::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_24_part_1() {
        let mut machine = parse_data(include_str!("../data/day_24_input.txt"));
        let result = machine.evaluate_till_completion();

        assert_eq!(result, 48806532300520);
    }
}