use crate::day23::day_23_common::*;
use std::collections::HashMap;
use itertools::Itertools;

fn day_23_part_2(graph: &HashMap<&str, Vec<&str>>) -> String {
    let mut largest = Vec::new();

    let keys: Vec<&&str> = graph.keys().collect();
    for first in keys {
        let mut set = vec![first];

        let first_friends = graph.get(first).unwrap();

        for second in first_friends {
            let second_friends = graph.get(second).unwrap();
            if set.iter().all(|node| second_friends.contains(node)) {
                set.push(second);
            }
        }
        if set.len() > largest.len() {
            largest = set;
        }
    }

    largest
        .into_iter()
        .sorted()
        .map(|slice| slice.to_string())
        .collect::<Vec<String>>()
        .join(",")
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_23_part_2_example_data() {
        let graph = parse_data(EXAMPLE_DATA);
        let password = day_23_part_2(&graph);

        assert_eq!(password, "co,de,ka,ta");
    }


    #[test]
    fn test_day_23_part_2() {
        let graph = parse_data(include_str!("../data/day_23_input.txt"));
        let password = day_23_part_2(&graph);

        assert_eq!(password, "ar,cd,hl,iw,jm,ku,qo,rz,vo,xe,xm,xv,ys");
    }
}