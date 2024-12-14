
use crate::day14::day_14_common::*;

pub fn get_christmas_tree(data: &str, width: i64, height: i64) -> Option<i64> {
    let mut robots = parse_data(data);

    for i in 0..width * height {
        if is_christmas_tree(&robots, width, height) {
            return Some(i);
        }

        step(&mut robots, width, height)
    }

    None
}

pub fn is_christmas_tree(robots: &Vec<Robot>, width:i64, height: i64) -> bool {
    let mut map = vec![vec![false; height as usize]; width as usize];

    for robot in robots {
        let (x, y) = robot.position;

        if map[x as usize][y as usize] {
            return false;
        }

        map[x as usize][y as usize] = true;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_christmas_tree() {
        let Some(index) = get_christmas_tree(include_str!("../data/day_14_input.txt"), 101, 103) else {
            panic!("Not found :( ");
        };

        assert_eq!(index, 8270);
    }
}