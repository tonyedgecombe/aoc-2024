pub fn parse_data() -> Vec<Vec<i32>> {
    let input = include_str!("./day_2_input.txt");
    let input = input.lines();

    let input: Vec<Vec<&str>> = input.map(|s| s.split_whitespace().collect()).collect();

    input.iter().map(|line| line.iter().map(|item| item.parse().unwrap()).collect()).collect()
}

pub fn differences(line: Vec<i32>) -> Vec<i32> {
    (0..line.len() - 1).map(|i| line[i] - line[i+1]).collect()
}

pub fn differences_in_range(differences: &Vec<i32>) -> bool {
    differences.iter().all(|d| d.abs() >= 1 && d.abs() <= 3)
}

pub fn same_sign(differences: &Vec<i32>) -> bool {
    differences.iter().all(|i| *i > 0) || differences.iter().all(|i| *i < 0)
}

mod test {
    use super::*;

    #[test]
    fn test_differences() {
        let data = vec![7, 6, 4, 2, 1];
        let result = differences(data);
        assert_eq!(result, [1, 2, 2, 1]);
    }

    #[test]
    fn test_same_sign() {
        let result = same_sign(&vec![-1, -2, -1]);
        assert!(result);

        let result = same_sign(&vec![1, 2, 1, 2]);
        assert!(result);

        let result = same_sign(&vec![-1, -2, -2, 1]);
        assert!(!result);

        let result = same_sign(&vec![-2, -1, 0, -2]);
        assert!(!result);
    }
}