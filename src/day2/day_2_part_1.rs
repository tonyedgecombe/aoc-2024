use super::day_2_common::*;

fn safe(data: &Vec<i32>) -> bool {
    same_sign(data) && differences_in_range(data)
}

pub fn day_2_part_1() -> usize {
    let data = parse_data();

    data
        .into_iter()
        .map(differences)
        .filter(safe)
        .count()
}

mod test {
    use super::*;

    #[test]
    fn test_day_2_part_1() {
        let result = day_2_part_1();
        assert_eq!(result, 490);
    }
}