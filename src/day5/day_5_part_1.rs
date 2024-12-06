use super::day_5_common::*;

fn day_5_part_1(rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .filter(|update| check_rules(update, rules))
        .map(|update| middle_page_number(update))
        .sum()
}


mod test {
    use crate::day5::day_5_common::parse_data;
    use super::*;

    #[test]
    fn test_day_5_part_1_example_data() {
        let (rules, updates) = parse_data(EXAMPLE_DATA);
        let result = day_5_part_1(&rules, &updates);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_day_5_part_1() {
        let (rules, updates) = parse_data(include_str!("./day_5_input.txt"));
        let result = day_5_part_1(&rules, &updates);
        assert_eq!(result, 4872);
    }
}