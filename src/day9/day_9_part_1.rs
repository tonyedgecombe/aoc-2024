use crate::day9::day_9_common::*;

pub fn compact(data: &Vec<i64>) -> Vec<i64> {
    let mut data = data.clone();
    let mut last_file = data.len() -1;

    for i in 0..data.len() {
        if data[i] == -1 {
            while data[last_file] == -1 {
                last_file -= 1;
            }

            if i >= last_file {
                break;
            }

            data.swap(i, last_file);
        }
    }

    data
}

fn day_9_part_1(data: &[u8]) -> i64 {
    let disk = parse_data(data);
    let disk = compact(&disk);
    checksum(&disk)
}

mod test {
    use super::*;

    #[test]
    fn test_compact_data() {
        let disk = parse_data(EXAMPLE_DATA);
        let disk = compact(&disk);

        assert_eq!(disk, [0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_day_9_part_1_example_data() {
        let checksum = day_9_part_1(EXAMPLE_DATA);
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn test_day_9_part_1() {
        let checksum = day_9_part_1(include_bytes!("../data/day_9_input.txt"));
        assert_eq!(checksum, 6211348208140);
    }
}