use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use crate::day19::day_19_common::*;

pub fn count_possible_designs(data: &'static [u8]) -> usize {
    let (towels, designs) = parse_data(data);

    designs.iter().filter(|design| memoized_matches(&towels, design)).count()
}

fn memoized_matches(towels: &[&[u8]], design: &'static [u8]) -> bool {
    static mut MATCHES: OnceLock<RwLock<HashMap<&[u8], bool>>> = OnceLock::new();

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

fn matches(towels: &[&[u8]], design: &'static[u8]) -> bool {
    if design.is_empty() {
        return true;
    }

    towels.iter().fold(false, |result, towel| {
        result | if design.starts_with(towel) {
            memoized_matches(towels, &design[towel.len()..])
        } else {
            false
        }
    })
}

fn day_19_part_1() -> usize {
    count_possible_designs(include_bytes!("../data/day_19_input.txt"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_19_part_1() {
        let count = day_19_part_1();
        assert_eq!(count, 251);
    }
}