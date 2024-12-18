use crate::day18::day_18_common::*;

pub fn day_18_part_2(data: &str, map_size: usize, start: usize) -> (usize, usize) {
    let lines = data.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();
    let length = lines.len();


    for n in start..=length {
        let map = parse_data(data, map_size, n);

        if count_steps(&map) == None {
            let pair: Vec<_> = lines[n - 1].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
            return(pair[0], pair[1]);
        }
    }

    panic!()
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let result = day_18_part_2(EXAMPLE_DATA, 7, 12);
        assert_eq!(result, (6,1));
    }
}