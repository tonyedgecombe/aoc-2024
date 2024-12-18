use crate::day18::day_18_common::*;

fn day_18_part_1() -> i64 {
    let map = parse_data(include_str!("../data/day_18_input.txt"), 71, 1024);
    count_steps(&map).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_18_part_1() {
        let steps = day_18_part_1();

        assert_eq!(steps, 296);
    }
}