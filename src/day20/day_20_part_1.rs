use crate::day20::day_20_common::*;

fn count_cheats(map: &Vec<Vec<u8>>, min_saving: i64) -> usize {
    get_cheats(map).into_iter().filter(|&saving| saving >= min_saving).count()
}

fn get_cheats(map: &Vec<Vec<u8>>) -> Vec<i64> {
    let mut cheat_savings = Vec::new();

    let steps = get_steps(map);

    for i in 0..steps.len() - 1 {
        let step = steps[i];

        for j in i+2..steps.len() {
            let dest_step = steps[j];

            if (step.0 == dest_step.0 && i64::abs(step.1 - dest_step.1) == 2) ||
                (step.1 == dest_step.1 && i64::abs(step.0 - dest_step.0) == 2) {
                if map[(step.0 + dest_step.0) as usize / 2][(step.1 + dest_step.1) as usize / 2] == b'#' {
                    cheat_savings.push((j - i - 2) as i64);
                }
            }
        }
    }

    cheat_savings
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_cheats() {
        let map = parse_data(EXAMPLE_DATA);
        let cheats = count_cheats(&map, 2);

        assert_eq!(cheats, 44);
    }
}