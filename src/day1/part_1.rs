fn part_1() -> i32 {
    let input = include_str!("./input.txt");
    let input = input.lines();
    let input: Vec<Vec<&str>> = input.map(|s| s.split_whitespace().collect()).collect();

    let mut left: Vec<i32> = input.iter().map(|sl| sl[0].parse().unwrap()).collect();
    let mut right: Vec<i32> = input.iter().map(|sl| sl[1].parse().unwrap()).collect();

    left.sort();
    right.sort();

    let zipped = left.iter().zip(right.iter());
    let differences = zipped.map(|(a, b)| (a-b).abs());

    differences.sum()
}

mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1();
        assert_eq!(result, 1651298);
    }
}