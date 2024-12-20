use crate::day20::day_20_common::*;

pub fn count_cheats(map: &Vec<Vec<u8>>, min_saving: i64) -> usize {
    get_cheats(map).into_iter().filter(|&saving| saving >= min_saving).count()
}

pub fn get_cheats(map: &Vec<Vec<u8>>) -> Vec<i64> {
    let mut cheat_savings = Vec::new();

    let steps = get_steps(map);

    for i in 0..steps.len() - 1 {
        let step = steps[i];

        for j in i+2..steps.len() {
            let dest_step = steps[j];

            let x_offset = i64::abs(step.0 - dest_step.0);
            let y_offset = i64::abs(step.1 - dest_step.1);
            let total_offset = x_offset + y_offset;

            if total_offset <= 20 {
                cheat_savings.push((j - i) as i64 - total_offset);
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
        let count = count_cheats(&map, 50);

        assert_eq!(count, 285)
    }

}