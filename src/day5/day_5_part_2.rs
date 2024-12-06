use crate::day5::day_5_common::*;

fn day_5_part_2(rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    updates
        .iter()
        .filter(|update| !check_rules(update, rules))
        .map(|update| fix_update(update, rules))
        .map(|update| middle_page_number(&update))
        .sum()
}

pub fn fix_update(update: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let mut update = update.to_vec();
    while apply_rules(&mut update, rules) {}

    update
}

pub fn apply_rules(update: &mut [i32], rules: &[(i32, i32)]) -> bool {
    rules.iter().any(|rule| apply_rule(update, *rule))
}

pub fn apply_rule(update: &mut [i32], rule: (i32, i32)) -> bool{
    let first = update.iter().position(|&x| x == rule.0);
    let second = update.iter().position(|&x| x == rule.1);

    match (first, second) {
        (Some(first), Some(second)) if first > second => {
            let tmp = update[first];
            update[first] = update[second];
            update[second] = tmp;
            true
        }
        (_, _) => false
    }
}


mod test {
    use crate::day5::day_5_common::*;
    use super::*;

    #[test]
    fn test_apply_rules() {
        let mut update = [2, 1, 4, 3, 5];
        let rules = [(1, 2), (3, 4)];

        apply_rules(&mut update, &rules);
        apply_rules(&mut update, &rules);

        assert_eq!(update, [1, 2, 3, 4, 5])
    }

    #[test]
    fn test_apply_rule() {
        let (rules, _) = parse_data(EXAMPLE_DATA);
        let mut update = [47,75,61,53,29];

        assert!(!check_rules(&update, &rules));

        apply_rule(&mut update, (75,47));

        assert_eq!(update, [75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_fix_update() {
        let mut update = [2, 1, 4, 3, 5];
        let rules = [(1, 2), (3, 4)];

        let update = fix_update(&mut update, &rules);

        assert_eq!(update, [1, 2, 3, 4, 5])
    }

    #[test]
    fn test_day_5_part_2_example_data() {
        let (rules, updates) = parse_data(EXAMPLE_DATA);
        let result = day_5_part_2(&rules, &updates);
        assert_eq!(result, 123);
    }

    #[test]
    fn test_day_5_part_2() {
        let (rules, updates) = parse_data(include_str!("./day_5_input.txt"));
        let result = day_5_part_2(&rules, &updates);
        assert_eq!(result, 5564);
    }
}