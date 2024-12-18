use crate::day17::day_17_common::*;

fn day_17_part_1() -> String {
    let mut machine = parse_data(include_str!("../data/day_17_input.txt"));

    let result = run_instructions(&mut machine);

    result.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_17_part_1() {
        let result = day_17_part_1();
        assert_eq!(result, "7,3,1,3,6,3,6,0,2");
    }
}