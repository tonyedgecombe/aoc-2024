use crate::day13::day_13_common::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_price() {
        let machines = parse_data(include_bytes!("../data/day_13_input.txt"));

        assert_eq!(total_price(&machines), 36838);

    }
}