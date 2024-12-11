mod test {
    use crate::day11::day_11_common::count_stones;

    #[test]
    fn test_day_11_part_2() {
        let result = count_stones(&[28, 4, 3179, 96938, 0, 6617406, 490, 816207], 75);
        assert_eq!(result, 225253278506288);
    }
}