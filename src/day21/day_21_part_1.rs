use crate::day21::day_21_common::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_complexity() {
        let codes = parse_data(include_bytes!("../data/day_21_input.txt"));
        let result = calculate_complexity(&codes, 4);
        assert_eq!(result, 123096);
    }
}