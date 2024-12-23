use crate::day23::day_23_common::*;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn day_23_part_1(graph: &HashMap<&str, Vec<&str>>) -> usize {
    let mut result: HashSet<Vec<&str>> = HashSet::new();

    let keys: Vec<&&str> = graph.keys().collect();

    for first in keys.iter().filter(|key| key.starts_with("t")) {
        let first_friends = graph.get(*first).unwrap();

        for second in first_friends {
            let second_friends = graph.get(second).unwrap();
            for third in second_friends {
                let third_friends = graph.get(third).unwrap();
                if third_friends.contains(first) {
                    let sorted: Vec<&str> = [**first, *second, *third].into_iter().sorted().collect();
                    result.insert(sorted);
                }
            }
        }

    }

    result.len()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sets_of_3_example_data() {
        let graph = parse_data(EXAMPLE_DATA);
        let result = day_23_part_1(&graph);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_sets_of_3() {
        let graph = parse_data(include_str!("../data/day_23_input.txt"));
        let result = day_23_part_1(&graph);

        assert_eq!(result, 1151);
    }
}