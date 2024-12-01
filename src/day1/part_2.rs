fn part_2() -> i32 {
    let input = include_str!("./input.txt");
    let input = input.lines();
    let input: Vec<Vec<&str>> = input.map(|s| s.split_whitespace().collect()).collect();

    let left = input.iter().map(|sl| sl[0].parse::<i32>().unwrap());
    let right: Vec<i32> = input.iter().map(|sl| sl[1].parse().unwrap()).collect();

    let similarities = left.map(|i| i * right.iter().filter(|j| i == **j).count() as i32);

    similarities.sum()
}

mod test {
    use super::*;

    #[test]
    fn test_part_2() {
        let result = part_2();
        assert_eq!(result, 21306195);
    }
}