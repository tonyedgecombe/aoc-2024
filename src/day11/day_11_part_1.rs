mod test {
    use crate::day11::day_11_common::count_stones;

    #[test]
    fn test_day_11_part_1() {
        let result = count_stones(&[28, 4, 3179, 96938, 0, 6617406, 490, 816207], 25);
        assert_eq!(result, 189167);
    }
}