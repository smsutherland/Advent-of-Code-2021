use std::cmp::Ordering;

pub fn run(lines: &[String]) -> (u32, u32) {
    let num_length = lines[0].len();

    let nums: Vec<u32> = lines
        .clone()
        .iter()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();

    let mut gamma = 0;
    let mut epsilon = 0;
    for c in (0..num_length)
        .rev()
        .map(|x| most_common_bit(&nums, x as u8))
    {
        assert!((0..=1).contains(&c));
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        gamma = gamma + c;
        epsilon = epsilon + 1 - c;
    }

    let mut oxygen_ratings = nums.clone();
    let mut carbon_ratings = nums.clone();

    for i in (0..num_length).rev() {
        let co = most_common_bit(&oxygen_ratings, i as u8);
        let cc = most_common_bit(&carbon_ratings, i as u8);
        oxygen_ratings = oxygen_ratings
            .drain(..)
            .filter(|x| (x >> i) & 1 == co)
            .collect();
        if carbon_ratings.len() > 1 {
            carbon_ratings = carbon_ratings
                .drain(..)
                .filter(|x| (x >> i) & 1 != cc)
                .collect();
        }
    }
    assert_eq!(oxygen_ratings.len(), 1);
    assert_eq!(carbon_ratings.len(), 1);

    let oxygen = oxygen_ratings[0];
    let carbon = carbon_ratings[0];

    (gamma * epsilon, oxygen * carbon)
}

fn most_common_bit(nums: &[u32], place: u8) -> u32 {
    assert!(place < 32);
    match nums
        .iter()
        .map(|x| (((x >> place) & 1) * 2) as i32 - 1)
        .sum::<i32>()
        .cmp(&0)
    {
        Ordering::Greater => 1,
        Ordering::Less => 0,
        Ordering::Equal => 1,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let a0 = most_common_bit(&vec![0b1100, 0b0101], 0);
        let a1 = most_common_bit(&vec![0b1100, 0b0101], 1);
        let a2 = most_common_bit(&vec![0b1100, 0b0101], 2);
        let a3 = most_common_bit(&vec![0b1100, 0b0101], 3);

        assert_eq!(a0, 1);
        assert_eq!(a1, 0);
        assert_eq!(a2, 1);
        assert_eq!(a3, 1);
    }

    #[test]
    fn works_on_odds() {
        let a0 = most_common_bit(&vec![0b1100, 0b0101, 0b1111], 0);
        let a1 = most_common_bit(&vec![0b1100, 0b0101, 0b1111], 1);
        let a2 = most_common_bit(&vec![0b1100, 0b0101, 0b1111], 2);
        let a3 = most_common_bit(&vec![0b1100, 0b0101, 0b1111], 3);

        assert_eq!(a0, 1);
        assert_eq!(a1, 0);
        assert_eq!(a2, 1);
        assert_eq!(a3, 1);
    }

    #[test]
    fn works_on_example() {
        let a = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let a0 = most_common_bit(&a, 0);
        let a1 = most_common_bit(&a, 1);
        let a2 = most_common_bit(&a, 2);
        let a3 = most_common_bit(&a, 3);
        let a4 = most_common_bit(&a, 4);

        assert_eq!(a0, 0);
        assert_eq!(a1, 1);
        assert_eq!(a2, 1);
        assert_eq!(a3, 0);
        assert_eq!(a4, 1);
    }

    #[test]
    fn test_case_1() {
        let strings = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        let result = run(&strings);
        assert_eq!(result, (198, 230));
    }

    #[test]
    fn binary_parse() {
        let binary_str = String::from("1100");
        let binary_int: u32 = u32::from_str_radix(&binary_str, 2).unwrap();

        assert_eq!(binary_int, 12);
    }
}
