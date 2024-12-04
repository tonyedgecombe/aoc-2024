pub fn day_4_part_2(data: &[u8]) -> usize {
    const SEARCH_STR: &[u8] = b"MAS";
    const SEARCH_STR_REV: &[u8] = b"SAM";

    let mut count = 0;

    let data = data.split(|&b| b == b'\n').collect::<Vec<&[u8]>>();

    let width = data[0].len() as i32;
    let height = data.len() as i32;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let x = x as usize;
            let y = y as usize;

            let forward = [data[x+1][y+1], data[x][y], data[x-1][y-1]];
            let backward = [data[x-1][y+1], data[x][y], data[x+1][y-1]];

            if (forward == SEARCH_STR || forward == SEARCH_STR_REV) && (backward == SEARCH_STR || backward == SEARCH_STR_REV) {
                count += 1;
            }
        }
    }

    count
}

mod test {
    use super::*;

    #[test]
    fn test_day_4_part_2() {
        let data = include_bytes!("./day_4_input.txt");
        let result = day_4_part_2(data);
        assert_eq!(result, 1886);
    }

    #[test]
    fn test_example_data() {
        let data = b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = day_4_part_2(data);
        assert_eq!(result, 9);
    }
}