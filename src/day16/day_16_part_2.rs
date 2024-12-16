use crate::day16::day_16_common::calculate_cost;

pub fn day_16_part_2() -> Option<i64> {
    match calculate_cost(include_bytes!("../data/day_16_input.txt")) {
        Some((_, tiles)) => Some(tiles),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(day_16_part_2().unwrap(), 631);
    }
}