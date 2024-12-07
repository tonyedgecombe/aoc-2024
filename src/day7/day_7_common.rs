#[derive(Debug, PartialEq)]
pub struct Equation {
    pub result: i64,
    pub operands: Vec<i64>,
}

#[derive(Clone)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

pub fn evaluate(equation: &Equation, operators: &Vec<Operator>) -> bool {
    let mut result = equation.operands[0];

    for i in 0..operators.len() {
        result = match operators[i] {
            Operator::Add => result + equation.operands[i + 1],
            Operator::Multiply => result * equation.operands[i + 1],
            Operator::Concatenate => concatenate(result, equation.operands[i + 1]),
        }
    }

    result == equation.result
}

fn concatenate(left: i64, right: i64) -> i64 {
    let mut log = 1;
    while log <= right {
        log *= 10;
    }

    left * log + right
}

fn parse_line(line: &str) -> Equation {
    let mut segments = line.split(":");
    let result = segments.next().unwrap().parse::<i64>().unwrap();
    let operands: Vec<i64> = segments.next().unwrap().trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();

    Equation{result, operands}
}

pub fn parse_data(data: &str) -> Vec<Equation> {
    data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .collect()
}

pub const EXAMPLE_DATA:&str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20

";

mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result = parse_line("190: 10 19");
        assert_eq!(result.result, 190);
        assert_eq!(result.operands, [10, 19]);
    }

    #[test]
    fn test_parse_line_long() {
        let result = parse_line("166733310603: 9 5 24 4 91 6 9 6 969 4 6");
        assert_eq!(result.result, 166733310603);
        assert_eq!(result.operands, [9, 5, 24, 4, 91, 6, 9, 6, 969, 4, 6]);
    }

    #[test]
    fn test_parse_lines() {
        let result = parse_data(EXAMPLE_DATA);
        assert_eq!(result.len(), 9);
        assert_eq!(result[0], Equation{result:190, operands:vec![10, 19]});
    }

    #[test]
    fn test_concatenate() {
        let result = concatenate(2345, 6789);
        assert_eq!(result, 23456789);
    }

    #[test]
    fn test_concatenate_bug() {
        let result = concatenate(4022, 1);
        assert_eq!(result, 40221);
    }
}