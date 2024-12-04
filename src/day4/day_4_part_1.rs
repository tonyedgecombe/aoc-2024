
fn day_4_part_1(data: &[u8]) -> usize {
    const SEARCH_STR : &[u8] = b"XMAS";

    let mut count = 0;

    let data = data.split(|&b| b == b'\n').collect::<Vec<&[u8]>>();

    let width = data[0].len() as i32;
    let height = data.len() as i32;

    for x in 0..width {
        for y in 0..height {
            for x_dir in -1..=1 {
                for y_dir in -1..=1 {
                    if x_dir == 0 && y_dir == 0 {
                        continue;
                    }

                    for i in 0..SEARCH_STR.len() as i32 {
                        let x = x + i * x_dir;
                        let y = y + i * y_dir;

                        if x < 0 || x >= width || y < 0 || y >= height {
                            break;
                        }

                        if data[x as usize][y as usize] != SEARCH_STR[i as usize] {
                            break;
                        }

                        if i == SEARCH_STR.len() as i32 - 1 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}



mod test {
    use super::*;

    #[test]
    fn test_day_4_part_1() {
        let data = include_bytes!("./day_4_input.txt");
        let result = day_4_part_1(data);
        assert_eq!(result, 2545);
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

        let result = day_4_part_1(data);
        assert_eq!(result, 18);
    }
}