use crate::day17::day_17_common::*;

fn day_17_part_2(data: &str) -> i64 {
    let mut machine = parse_data(data);
    let mut expected = machine.program.clone();
    expected.reverse();

    find_solution(&mut machine, &expected, 0)
}

fn find_solution(machine: &mut Machine, expected: &[i64], a: i64) -> i64 {
    if expected.is_empty() {
        return a;
    }

    let mut solutions = Vec::new();

    for i in 0..8 {
        let a = a * 8 + i;

        let result = run_with_a(machine, a);
        if result[0] == expected[0] {
            solutions.push(find_solution(machine, &expected[1..], a));
        }
    }

    solutions.into_iter().min().unwrap_or(i64::MAX)
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_17_part_2_example_data() {
        let result = day_17_part_2(EXAMPLE_DATA_2);

        assert_eq!(result, 117440);
    }

    #[test]
    fn test_day_17_part_2() {
        let result = day_17_part_2(include_str!("../data/day_17_input.txt"));

        let mut machine = parse_data(include_str!("../data/day_17_input.txt"));
        machine.register_a = result;

        let result = run_instructions(&mut machine);

        assert_eq!(result, machine.program);
    }
}