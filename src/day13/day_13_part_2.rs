use crate::day13::day_13_common::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_price() {
        let machines = parse_data(include_bytes!("../data/day_13_input.txt"));
        let new_machines: Vec<_> = machines
            .into_iter()
            .map(|machine| Machine {
                prize: (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000),
                ..machine
            })
            .collect();

        assert_eq!(total_price(&new_machines), 83029436920891);
    }
}