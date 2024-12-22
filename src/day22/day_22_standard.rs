pub fn parse_data(data: &str) -> Vec<i64> {
    data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn next(secret: i64) -> i64 {
    let secret = (secret ^ secret * 64) % 16777216;
    let secret = (secret ^ secret / 32) % 16777216;
    let secret = (secret ^ secret * 2048) % 16777216;

    secret
}

pub fn generate_secret_numbers(secret: i64, count: usize) -> Vec<i64> {
    let mut secret = secret;
    let mut result = Vec::new();

    for _ in 0..count {
        result.push(secret);
        secret = next(secret);
    }

    result
}

pub fn two_thousandth(secret: i64) -> i64 {
    let mut secret = secret;

    for _ in 0..2000 {
        secret = next(secret);
    }

    secret
}



pub const EXAMPLE_DATA: &str = "
1
10
100
2024";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(next(123), 15887950);
        assert_eq!(next(15887950), 16495136);
        assert_eq!(next(16495136), 527345);
        assert_eq!(next(527345), 704524);
        assert_eq!(next(704524), 1553684);
        assert_eq!(next(1553684), 12683156);
        assert_eq!(next(12683156), 11100544);
        assert_eq!(next(11100544), 12249484);
        assert_eq!(next(12249484), 7753432);
        assert_eq!(next(7753432), 5908254);
    }

    #[test]
    fn test_two_thousandth() {
        assert_eq!(two_thousandth(1), 8685429);
        assert_eq!(two_thousandth(10), 4700978);
        assert_eq!(two_thousandth(100), 15273692);
        assert_eq!(two_thousandth(2024), 8667524);
    }

    #[test]
    fn test_parse() {
        let result = parse_data(EXAMPLE_DATA);
        assert_eq!(result, [1, 10, 100, 2024]);
    }

    #[test]
    fn test_generate_secret_numbers() {
        let numbers = generate_secret_numbers(123, 10);
        assert_eq!(numbers, [123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432]);
    }
}