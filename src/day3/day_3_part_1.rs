use super::day_3_common::Parser;

fn day_3_part_1() -> i32 {
    let data = include_bytes!("./day_3_input.txt");
    let mut parser = Parser::new(data);
    parser.sum_expressions_all()
}

mod test {
    use super::*;

    #[test]
    fn test_day_3_part_1() {
        assert_eq!(day_3_part_1(), 173785482);
    }
}