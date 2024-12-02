use super::day_2_common::*;

fn remove_item(items: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut result = items.clone();
    result.remove(index);
    result
}

fn safe(items: &Vec<i32>) -> bool {
    (0..items.len()).any(|i| {
        let shortened = remove_item(&items, i);
        let differences = differences(shortened);

        differences_in_range(&differences) && same_sign(&differences)
    })
}

fn day_2_part_2() -> usize {
    let data = parse_data();

    data
        .into_iter()
        .filter(safe)
        .count()
}



mod test {
    use super::*;

    #[test]
    fn test_remove_item() {
        assert_eq!(remove_item(&vec![1, 2, 3], 0), vec![2, 3]);
        assert_eq!(remove_item(&vec![1, 2, 3], 1), vec![1, 3]);
        assert_eq!(remove_item(&vec![1, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn test_safe() {
        assert!(safe(&vec![1, 1, 2, 3, 4]));
        assert!(safe(&vec![1, -1, 2, 3, 4]));
        assert!(!safe(&vec![3, 1, 3, 1]));

        assert!(safe(&vec![1, 1, 2, 3]));
        assert!(safe(&vec![1, 1, 2, 3, 4]));
        assert!(!safe(&vec![0, 1, 2, 7, 8]));

    }

    #[test]
    fn test_day_2_part_2() {
        assert_eq!(day_2_part_2(), 536);
    }
}