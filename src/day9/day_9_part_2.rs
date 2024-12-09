use crate::day9::day_9_common::*;

pub fn day_2_compact(data: &Vec<i64>) -> Vec<i64> {
    let mut result = data.clone();

    let last_id = *result.iter().rfind(|&&x| x != -1).unwrap();
    let mut free_blocks = find_free_blocks(&result);

    let mut file_pos = result.len() - 1;
    for id in (0..=last_id).rev() {
        while result[file_pos] != id && file_pos > 0{
            file_pos -= 1;
        }
        while result[file_pos] == id && file_pos > 0{
            file_pos -= 1;
        }
        file_pos += 1;

        let mut file_length = 0;
        while file_pos + file_length < result.len() && result[file_pos + file_length] == id {
            file_length += 1;
        }

        let Some(free_block_pos) = free_blocks.iter().position(|&(_p, l)| l >= file_length) else {
            continue;
        };

        let free_block = free_blocks[free_block_pos];

        if free_block.0 >= file_pos {
            continue;
        }

        for i in 0..file_length {
            result.swap(file_pos + i, free_block.0 + i);
        }

        free_blocks[free_block_pos].0 += file_length;
        free_blocks[free_block_pos].1 -= file_length;
    }

    result
}

fn find_free_blocks(data: &Vec<i64>) -> Vec<(usize, usize)> {
    let mut result = vec![];

    let mut i = 0;
    while i < data.len() {
        if data[i] == -1 {
            let mut length = 0;
            while i + length < data.len() && data[i + length] == -1 {
                length += 1;
            }

            result.push((i, length));
            i += length;
        } else {
            i += 1;
        }
    }

    result
}


fn day_9_part_2(data: &[u8]) -> i64 {
    let disk = parse_data(data);
    let disk = day_2_compact(&disk);
    checksum(&disk)
}

mod test {
    use super::*;

    #[test]
    fn test_find_free_blocks() {
        let disk = parse_data(EXAMPLE_DATA);
        let free_blocks = find_free_blocks(&disk);

        assert_eq!(free_blocks.len(), 8);
        assert_eq!(free_blocks[0], (2, 3));
    }

    #[test]
    fn test_compact_data() {
        let disk = parse_data(EXAMPLE_DATA);
        let disk = day_2_compact(&disk);

        let expected = b"00992111777.44.333....5555.6666.....8888.."
            .iter()
            .map(|&c| if c == b'.' {-1} else {(c - b'0') as i64 })
            .collect::<Vec<i64>>();

        assert_eq!(disk, expected);
    }

    #[test]
    fn test_day_9_part_2_example_data() {
        let checksum = day_9_part_2(EXAMPLE_DATA);
        assert_eq!(checksum, 2858);
    }

    #[test]
    fn test_day_9_part_2() {
        let checksum = day_9_part_2(include_bytes!("../data/day_9_input.txt"));
        assert_eq!(checksum, 6239783302560);
    }
}