use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use crate::day19::day_19_common::parse_data;

pub fn count_combinations(data: &'static [u8]) -> usize {
    let (towels, designs) = parse_data(data);

    designs.iter().map(|design| memoized_matches(&towels, design)).sum()
}

fn memoized_matches(towels: &[&[u8]], design: &'static [u8]) -> usize {
    static mut MATCHES: OnceLock<RwLock<HashMap<&[u8], usize>>> = OnceLock::new();

    unsafe {
        {
            let _ = MATCHES.get_or_init(|| {RwLock::new(HashMap::new())});
            let map = MATCHES.get_mut().unwrap().read().unwrap();


            if let Some(count) = map.get(&design) {
                return *count;
            }
        }

        let result = matches(towels, design);
        let mut map = MATCHES.get_mut().unwrap().write().unwrap();
        map.insert(design, result);

        result
    }
}

fn matches(towels: &[&[u8]], design: &'static[u8]) -> usize {
    if design.is_empty() {
        return 1;
    }

    towels.iter().fold(0, |result, towel| {
        result + if design.starts_with(towel) {
            memoized_matches(towels, &design[towel.len()..])
        } else {
            0
        }
    })
}

fn day_19_part_2() -> usize {
    count_combinations(include_bytes!("../data/day_19_input.txt"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let total = day_19_part_2();
        assert_eq!(total, 616957151871345);
    }
}