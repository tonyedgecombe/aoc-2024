
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

pub fn memoized_count(stone: u64, iterations: usize) -> usize {
    static mut COUNTS: OnceLock<RwLock<HashMap<u128, usize>>> = OnceLock::new();
    let key = (stone as u128) << 64 | (iterations as u128);

    unsafe {
        {
            let _ = COUNTS.get_or_init(|| {RwLock::new(HashMap::new())});
            let counts = COUNTS.get_mut().unwrap().read().unwrap();


            if let Some(count) = counts.get(&key) {
                return *count;
            }
        }

        let result = count(stone, iterations);
        let mut counts = COUNTS.get_mut().unwrap().write().unwrap();
        counts.insert(key, result);

        result
    }
}

pub fn count(stone: u64, iterations: usize) -> usize {
    if iterations == 0 {
        return 1;
    }

    if stone == 0 {
        return memoized_count(1, iterations - 1)
    }

    let il = stone.ilog10();
    if il % 2 == 1 {
        let length = il + 1;
        let divisor = 10_u64.pow((length + 1) / 2);

        memoized_count(stone / divisor, iterations - 1) + memoized_count(stone % divisor, iterations - 1)
    } else {
        memoized_count(stone * 2024, iterations - 1)
    }
}

pub fn count_stones(stones: &[u64], iterations: usize) -> usize {
    stones.iter().map(|stone| memoized_count(*stone, iterations)).sum()
}

mod test {
    use super::*;

    #[test]
    fn test_count_termination() {
        assert_eq!(count(0, 0), 1);
    }

    // Test one iteration
    #[test]
    fn test_count_0_1() {
        assert_eq!(count(0, 1), 1);
    }

    #[test]
    fn test_count_1_1() {
        assert_eq!(count(1, 1), 1);
    }

    #[test]
    fn test_count_10_1() {
        assert_eq!(count(10, 1), 2);
    }

    // Test two iterations
    #[test]
    fn test_count_1111_2() {
        assert_eq!(count(1111, 2), 4);
    }

    #[test]
    fn count_stones_6() {
        let result = count_stones(&[125, 17], 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn count_stones_25() {
        let result = count_stones(&[125, 17], 25);
        assert_eq!(result, 55312);
    }
}