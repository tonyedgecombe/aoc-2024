use std::collections::{HashMap, HashSet};
use crate::day22::day_22_standard::*;

pub struct Buyer {
    secrets: Vec<i64>,
    offerings: Vec<i64>,
    offering_differences: Vec<i64>,
    sequences: Vec<[i64; 4]>,
    sequence_to_price: HashMap<[i64; 4], i64>,
}

impl Buyer {
    fn new(seed: i64, count: usize) -> Buyer {
        let secrets = generate_secret_numbers(seed, count);
        let offerings: Vec<i64> = secrets.iter().map(|secret| secret % 10).collect();
        let offering_differences: Vec<i64> = (1..secrets.len())
            .map(|i| (offerings[i]) - (offerings[i - 1]))
            .collect();

        let sequences = (0..offering_differences.len() - 3)
            .map(|index| array(&offering_differences[index..index+4]))
            .collect();

        let mut sequence_to_price = HashMap::new();
        for index in 0..offering_differences.len() - 3 {
            let key = array(&offering_differences[index..index+4]);
            if !sequence_to_price.contains_key(&key) {
                sequence_to_price.insert(key, offerings[index + 4]);
            }
        }

        Buyer {
            secrets,
            offerings,
            offering_differences,
            sequences,
            sequence_to_price,
        }
    }

    fn offerings(&self) -> &Vec<i64> {
        &self.offerings
    }

    fn offering_differences(&self) -> &Vec<i64> {
        &self.offering_differences
    }

    fn sequences(&self) -> &Vec<[i64; 4]> {
        &self.sequences
    }

    fn price_from_sequence(&self, sequence: &[i64; 4]) -> i64 {
        *self.sequence_to_price.get(sequence).unwrap_or(&0)
    }
}

pub struct Buyers {
    buyers: Vec<Buyer>,
}

impl Buyers {
    pub(crate) fn new(seeds: &[i64], count: usize) -> Buyers {
        let buyers: Vec<Buyer> = seeds
            .into_iter()
            .map(|seed| Buyer::new(*seed, count))
            .collect();

        Buyers {
            buyers
        }
    }

    fn price_from_sequence(&self, sequence: &[i64; 4]) -> i64 {
        self.buyers.iter().map(|buyer| buyer.price_from_sequence(sequence)).sum::<i64>()
    }

    fn merged_unique_sequences(&self) -> HashSet<[i64; 4]> {
        let mut unique_sequences: HashSet<[i64; 4]> = HashSet::new();

        for buyer in &self.buyers {
            for sequence in buyer.sequences() {
                unique_sequences.insert(*sequence);
            }
        }

        unique_sequences
    }

    pub(crate) fn most_bananas(&self) -> i64 {
        let mut max = 0;

        for sequence in self.merged_unique_sequences() {
            let price = self.price_from_sequence(&sequence);
            if price > max {
                max = price;
            }
        }

        max
    }
}

fn array<T: Copy, const N: usize>(slice: &[T]) -> [T; N] {
    slice.try_into().expect("Slice has the wrong length")
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_buyer_new() {
        let buyer = Buyer::new(123, 10);

        assert_eq!(buyer.secrets.len(), 10);
        assert_eq!(buyer.secrets[1], 15887950);
    }

    #[test]
    fn test_buyer_offerings() {
        let buyer = Buyer::new(123, 10);
        let offerings = buyer.offerings();

        assert_eq!(offerings, &[3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }

    #[test]
    fn test_buyer_offering() {
        let buyer = Buyer::new(123, 10);
        let offering_differences = buyer.offering_differences();

        assert_eq!(offering_differences, &[-3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }

    #[test]
    fn test_sequences() {
        let buyer = Buyer::new(123, 10);
        let sequences = buyer.sequences();

        assert_eq!(sequences.len(), 6);
        assert_eq!(sequences[0].to_vec(), &[-3, 6, -1, -1]);
    }

    #[test]
    fn test_price_from_sequence() {
        let buyer = Buyer::new(123, 10);
        let price = buyer.price_from_sequence(&[-1, -1, 0, 2]);
        assert_eq!(price, 6);
    }

    #[test]
    fn test_price_from_sequences() {
        let buyer = Buyer::new(1, 2000);
        let price = buyer.price_from_sequence(&[-2,1,-1,3]);
        assert_eq!(price, 7);

        let buyer = Buyer::new(2, 2000);
        let price = buyer.price_from_sequence(&[-2,1,-1,3]);
        assert_eq!(price, 7);

        let buyer = Buyer::new(3, 2000);
        let price = buyer.price_from_sequence(&[-2,1,-1,3]);
        assert_eq!(price, 0);

        let buyer = Buyer::new(2024, 2000);
        let price = buyer.price_from_sequence(&[-2,1,-1,3]);
        assert_eq!(price, 9);
    }

    #[test]
    fn test_buyers_new() {
        let buyers = Buyers::new(&[1,2,3,2024], 2000);
        assert_eq!(buyers.buyers.len(), 4);
    }

    #[test]
    fn test_buyers_price_from_seed() {
        let buyers = Buyers::new(&[1,2,3,2024], 2000);
        let price = buyers.price_from_sequence(&[-2,1,-1,3]);
        assert_eq!(price, 23);
    }

    #[test]
    fn test_buyers_unique_sequences() {
        let buyers = Buyers::new(&[1, 1], 2000);
        let sequences = buyers.merged_unique_sequences();
        assert_eq!(sequences.len(), 1932);
    }

    #[test]
    fn test_most_bananas() {
        let buyers = Buyers::new(&[1,2,3,2024], 2000);
        let banana_count = buyers.most_bananas();

        assert_eq!(banana_count, 23);
    }

    // #[test]
    // fn test_day_22_part_2() {
    //     let seeds = parse_data(include_str!("../data/day_22_input.txt"));
    //     let buyers = Buyers::new(&seeds, 2000);
    //     let banana_count = buyers.most_bananas();
    //
    //     assert_eq!(banana_count, 1938);
    // }
}