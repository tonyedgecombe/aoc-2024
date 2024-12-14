use crate::day14::day_14_common::*;

pub fn day_14_part_1(data: &str, width: i64, height: i64) -> usize {
    let mut robots = parse_data(data);

    (0..100).for_each(|_| {step(&mut robots, width, height);});

    safety_factor(&mut robots, width, height)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_14_part_1() {
        let safety_factor = day_14_part_1(include_str!("../data/day_14_input.txt"), 101, 103);

        assert_eq!(safety_factor, 230436441);
    }
}