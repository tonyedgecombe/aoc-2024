
pub fn parse_data(data: &[u8]) -> Vec<i64> {
    let mut result = vec![];

    for id in 0..= data.len() / 2 {
        let pos = id * 2;
        let file_length = data[pos] - b'0';

        for _ in 0..file_length {
            result.push(id as i64);
        }

        let pos = pos + 1;
        if pos >= data.len() {
            break;
        }

        let free_space_length = data[pos] - b'0';
        for _ in 0..free_space_length {
            result.push(-1);
        }
    }

    result
}

pub fn checksum(data: &[i64]) -> i64 {
    data
        .iter()
        .enumerate()
        .filter(|(_, &id)| id != -1)
        .map(|(idx, x)| idx as i64 * x)
        .sum()
}

pub const EXAMPLE_DATA: &[u8] = b"2333133121414131402";

mod test {
    use super::*;

    #[test]
    fn test_parse_example_data() {
        let result = parse_data(EXAMPLE_DATA);
        assert_eq!(result, [0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9]);
    }

}