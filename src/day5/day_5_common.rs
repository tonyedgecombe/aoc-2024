pub fn parse_data(data: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let lines = data.split("\n");

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        if line.contains('|') {
            let mut parts = line.split("|");
            let rule = (parts.next().unwrap().parse::<i32>().unwrap(),
                        parts.next().unwrap().parse::<i32>().unwrap()
            );

            rules.push(rule);
        } else {
            let page_numbers = line.split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            updates.push(page_numbers);
        }
    }

    (rules, updates)
}

pub fn check_rules(update: &[i32], rules: &[(i32, i32)]) -> bool {
    rules.iter().all(|rule| check_rule(update, *rule))
}

fn check_rule(update: &[i32], rule: (i32, i32)) -> bool {
    let first = update.iter().position(|&x| x == rule.0);
    let second = update.iter().position(|&x| x == rule.1);

    match (first, second) {
        (Some(first), Some(second)) => first < second,
        (_, _) => true,
    }
}

pub fn middle_page_number(update: &[i32]) -> i32 {
    update[update.len() / 2]
}


pub const EXAMPLE_DATA: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

mod test {
    use super::*;

    #[test]
    fn test_middle_page_number() {
        assert_eq!(middle_page_number(&[75,47,61,53,29]), 61);
    }
    #[test]
    fn test_check_rules() {
        let (rules, _) = parse_data(EXAMPLE_DATA);
        assert!(check_rules(&[75,47,61,53,29], &rules));
    }

    #[test]
    fn test_check_rule() {
        assert!(check_rule(&[75,47,61,53,29], (75, 47)));
        assert!(!check_rule(&[75,47,61,53,29], (47, 75)));
    }

    #[test]
    fn test_example_data() {
        let (rules, updates) = parse_data(EXAMPLE_DATA);
        assert_eq!(rules.len(), 21);
        assert_eq!(updates.len(), 6);
    }

}