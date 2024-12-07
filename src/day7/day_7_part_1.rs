use crate::day7::day_7_common::*;

fn check_equation_two_operators(equation: &Equation) -> bool {
    let operator_count = equation.operands.len() - 1;
    let mut operators = vec![Operator::Add; operator_count];

    for _ in 0..2i64.pow(operator_count as u32) {
        if evaluate(&equation, &operators) {
            return true;
        }

        for i in (0..operator_count).rev() {
            match operators[i] {
                Operator::Add => {
                    operators[i] = Operator::Multiply;
                    break;
                }
                Operator::Multiply => {
                    operators[i] = Operator::Add;
                }
                Operator::Concatenate => panic!()
            }
        }
    }

    false
}




fn day_7_part_1(data: &str) -> i64 {
    parse_data(data)
        .iter()
        .filter(|e| check_equation_two_operators(e))
        .map(|e| e.result)
        .sum()
}



mod test {
    use super::*;

    #[test]
    fn test_check_equation_success() {
        let equation = Equation { result: 190, operands: vec![10, 19] };
        assert!(check_equation_two_operators(&equation));
    }

    #[test]
    fn test_check_equation_fail() {
        let equation = Equation { result: 83, operands: vec![17, 15] };
        assert!(!check_equation_two_operators(&equation));
    }

    #[test]
    fn test_day_7_part_1_example_data() {
        let result = day_7_part_1(EXAMPLE_DATA);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_day_7_part_1() {
        let result = day_7_part_1(include_str!("../data/day_7_input.txt"));
        assert_eq!(result, 4364915411363i64);
    }
}