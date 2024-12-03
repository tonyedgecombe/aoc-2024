use crate::day3::day_3_common::Parser;

fn day_3_part_2() -> i32 {
    let data = include_bytes!("./day_3_input.txt");
    let mut parser = Parser::new(data);
    parser.sum_expressions_doos()
}

mod test {
    use super::*;

    #[test]
    fn test_day_3_part_2() {
        assert_eq!(day_3_part_2(), 83158140);
    }
}