use crate::day16::day_16_common::*;

pub fn day_16_part_1() -> Option<i64> {
    match calculate_cost(include_bytes!("../data/day_16_input.txt")) {
        Some((cost, _)) => Some(cost),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_day_16_part_1() {
        let cost = day_16_part_1().unwrap();
        assert_eq!(cost, 134588);
    }
}