use crate::day7::day_7_common::*;

fn check_equation_three_operators(equation: &Equation) -> bool {
    let operator_count = equation.operands.len() - 1;
    let mut operators = vec![Operator::Add; operator_count];

    for _ in 0..3i64.pow(operator_count as u32) {
        if evaluate(&equation, &operators) {
            return true;
        }

        for i in 0..operator_count {
            match operators[i] {
                Operator::Add => {
                    operators[i] = Operator::Multiply;
                    break;
                }
                Operator::Multiply => {
                    operators[i] = Operator::Concatenate;
                    break;
                }
                Operator::Concatenate => {
                    operators[i] = Operator::Add;
                }
            }
        }
    }

    false
}

fn day_7_part_2(data: &str) -> i64 {
    parse_data(data)
        .iter()
        .filter(|equation| check_equation_three_operators(equation))
        .map(|equation| equation.result)
        .sum()
}

mod test {
    use super::*;

    #[test]
    fn test_check_multiplication_success() {
        let equation = Equation { result: 190, operands: vec![10, 19] };
        assert!(check_equation_three_operators(&equation));
    }

    #[test]
    fn test_check_addition_success() {
        let equation = Equation { result: 29, operands: vec![10, 19] };
        assert!(check_equation_three_operators(&equation));
    }

    #[test]
    fn test_check_equation_fail() {
        let equation = Equation { result: 83, operands: vec![17, 15] };
        assert!(!check_equation_three_operators(&equation));
    }

    #[test]
    fn test_concatenation() {
        let equation = Equation { result: 12345, operands: vec![12, 345] };
        assert!(check_equation_three_operators(&equation));
    }

    #[test]
    fn test_day_7_part_2_example_data() {
        let result = day_7_part_2(EXAMPLE_DATA);
        assert_eq!(result, 11387);
    }

    #[test]
    fn test_day_7_part_2() {
        let result = day_7_part_2(include_str!("../data/day_7_input.txt"));
        assert_eq!(result, 38322057216320);
    }
}