use std::cmp::{Eq, PartialEq};
use std::fmt::{self, Debug, Display};
use std::ops::Add;
use std::str::FromStr;
// use std::io::repeat;

#[derive(PartialEq, Eq, Clone)]
struct SnailfishNumber {
    numbers: Vec<(u8, u8)>,
}

impl SnailfishNumber {
    fn new(numbers: Vec<(u8, u8)>) -> Self {
        let mut result = Self { numbers };
        result.reduce();
        result
    }

    fn reduce(&mut self) {
        'main_loop: loop {
            for (i, num) in self.numbers.iter().enumerate() {
                if num.1 == 5 {
                    self.explode(i);
                    continue 'main_loop;
                }
            }
            for (i, num) in self.numbers.iter().enumerate() {
                if num.0 >= 10 {
                    self.split(i);
                    continue 'main_loop;
                }
            }
            break;
        }
    }

    fn explode(&mut self, i: usize) {
        let num_1 = self.numbers[i];
        let num_2 = self.numbers[i + 1];

        assert_eq!(num_1.1, num_2.1);
        let depth = num_1.1;

        if i != 0 {
            if let Some(val) = self.numbers.get_mut(i - 1) {
                val.0 += num_1.0;
            }
        }
        if let Some(val) = self.numbers.get_mut(i + 2) {
            val.0 += num_2.0;
        }

        self.numbers.remove(i + 1);
        self.numbers[i] = (0, depth - 1);
    }

    fn split(&mut self, i: usize) {
        let num = self.numbers[i];
        self.numbers[i] = (num.0 / 2, num.1 + 1);
        self.numbers
            .insert(i + 1, (num.0 / 2 + num.0 % 2, num.1 + 1));
    }

    fn magnitude(&self) -> u64 {
        const MAX_DEPTH: u8 = 4;
        let mut progress: u8 = 0;
        let mut mag = 0;

        for num in &self.numbers {
            let (val, depth) = *num;

            let mut num_3 = 0;
            let mut num_2 = 0;
            for i in (MAX_DEPTH - depth)..MAX_DEPTH {
                if (progress >> i) & 1 == 1 {
                    num_2 += 1;
                } else {
                    num_3 += 1;
                }
            }

            let depth_factor = u64::pow(3, num_3) * u64::pow(2, num_2);
            mag += depth_factor * val as u64;

            progress += 1 << (MAX_DEPTH - depth);
        }
        assert_eq!(progress, 16);
        mag
    }
}

impl FromStr for SnailfishNumber {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Err(String::from("str must start with '['"));
        }
        if !s.ends_with(']') {
            return Err(String::from("str must end with ']'"));
        }

        let mut numbers = Vec::new();
        let mut depth = 0;
        for c in s.chars() {
            if c == '[' {
                depth += 1;
            }
            if c == ']' {
                depth -= 1;
            }
            if depth < 0 {
                return Err(String::from("bracket depth cannot be negative"));
            }

            if let Some(num) = c.to_digit(10) {
                numbers.push((num as u8, depth as u8));
            }
        }
        if depth != 0 {
            return Err(String::from("unmatched brackets"));
        }

        Ok(Self::new(numbers))
    }
}

impl Add<&SnailfishNumber> for SnailfishNumber {
    type Output = SnailfishNumber;
    fn add(self, rhs: &Self) -> SnailfishNumber {
        let mut numbers = self.numbers.clone();
        numbers.append(&mut rhs.numbers.clone());
        for num in &mut numbers {
            num.1 += 1;
        }

        SnailfishNumber::new(numbers)
    }
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;
    fn add(self, rhs: Self) -> SnailfishNumber {
        let mut numbers = self.numbers.clone();
        numbers.append(&mut rhs.numbers.clone());
        for num in &mut numbers {
            num.1 += 1;
        }

        SnailfishNumber::new(numbers)
    }
}

impl Add for &SnailfishNumber {
    type Output = SnailfishNumber;
    fn add(self, rhs: Self) -> SnailfishNumber {
        let mut numbers = self.numbers.clone();
        numbers.append(&mut rhs.numbers.clone());
        for num in &mut numbers {
            num.1 += 1;
        }

        SnailfishNumber::new(numbers)
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        const MAX_DEPTH: u8 = 4;
        let mut progress: u8 = 0;
        let mut prev_depth = 0;

        for num in &self.numbers {
            let (val, depth) = *num;
            let mut comma_depth = 0;
            for i in 0..MAX_DEPTH {
                if progress >> i & 1 == 1 {
                    comma_depth = MAX_DEPTH - i;
                    break;
                }
            }

            if prev_depth > comma_depth {
                write!(f, "{}", "]".repeat((prev_depth - comma_depth) as usize))?;
            }
            if progress != 0 {
                write!(f, ",")?;
            }
            if depth > comma_depth {
                write!(f, "{}", "[".repeat((depth - comma_depth) as usize))?;
            }
            write!(f, "{}", val)?;

            prev_depth = depth;
            progress += 1 << (MAX_DEPTH - depth);
        }
        write!(f, "{}", "]".repeat((prev_depth) as usize))?;
        assert_eq!(progress, 16);

        Ok(())
    }
}

impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self)?;
        Ok(())
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut nums: Vec<SnailfishNumber> = Vec::new();
    for line in lines {
        nums.push(line.parse().unwrap());
    }

    let mut part_1_num = nums[0].clone();
    for num in &nums[1..] {
        part_1_num = part_1_num + num;
    }
    let part_1 = part_1_num.magnitude();

    let mut part_2 = 0;
    for (i, num_1) in nums.iter().enumerate() {
        for (j, num_2) in nums.iter().enumerate() {
            if i == j {
                continue;
            }

            let mag = (num_1 + num_2).magnitude();
            if mag > part_2 {
                part_2 = mag;
            }
        }
    }

    (part_1, part_2)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parsing() {
        let raw_str_1 = "[1,2]";
        assert_eq!(
            raw_str_1.parse::<SnailfishNumber>().unwrap().numbers,
            vec![(1, 1), (2, 1)]
        );

        let raw_str_2 = "[[1,2],3]";
        assert_eq!(
            raw_str_2.parse::<SnailfishNumber>().unwrap().numbers,
            vec![(1, 2), (2, 2), (3, 1)]
        );

        let raw_str_3 = "[9,[8,7]]";
        assert_eq!(
            raw_str_3.parse::<SnailfishNumber>().unwrap().numbers,
            vec![(9, 1), (8, 2), (7, 2)]
        );

        let raw_str_4 = "[[1,9],[8,5]]";
        assert_eq!(
            raw_str_4.parse::<SnailfishNumber>().unwrap().numbers,
            vec![(1, 2), (9, 2), (8, 2), (5, 2)]
        );

        let raw_str_4 = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        assert_eq!(
            raw_str_4.parse::<SnailfishNumber>().unwrap().numbers,
            vec![
                (1, 4),
                (3, 4),
                (5, 4),
                (3, 4),
                (1, 4),
                (3, 4),
                (8, 4),
                (7, 4),
                (4, 4),
                (9, 4),
                (6, 4),
                (9, 4),
                (8, 4),
                (2, 4),
                (7, 4),
                (3, 4)
            ]
        );
    }

    #[test]
    fn exploding() {
        let raw_str_1_a = "[[[[[9,8],1],2],3],4]";
        let raw_str_1_b = "[[[[0,9],2],3],4]";
        assert_eq!(
            raw_str_1_a.parse::<SnailfishNumber>().unwrap(),
            raw_str_1_b.parse::<SnailfishNumber>().unwrap()
        );

        let raw_str_2_a = "[7,[6,[5,[4,[3,2]]]]]";
        let raw_str_2_b = "[7,[6,[5,[7,0]]]]";
        assert_eq!(
            raw_str_2_a.parse::<SnailfishNumber>().unwrap(),
            raw_str_2_b.parse::<SnailfishNumber>().unwrap()
        );

        let raw_str_2_a = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let raw_str_2_b = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        assert_eq!(
            raw_str_2_a.parse::<SnailfishNumber>().unwrap(),
            raw_str_2_b.parse::<SnailfishNumber>().unwrap()
        );
    }

    #[test]
    fn addition() {
        let num_1: SnailfishNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let num_2: SnailfishNumber = "[1,1]".parse().unwrap();

        assert_eq!(
            (num_1 + num_2),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<SnailfishNumber>()
                .unwrap()
        );

        let num_1: SnailfishNumber = "[1,1]".parse().unwrap();
        let num_2: SnailfishNumber = "[2,2]".parse().unwrap();
        let num_3: SnailfishNumber = "[3,3]".parse().unwrap();
        let num_4: SnailfishNumber = "[4,4]".parse().unwrap();

        assert_eq!(
            (num_1 + num_2 + num_3 + num_4),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .parse::<SnailfishNumber>()
                .unwrap()
        );

        let num_1: SnailfishNumber = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse().unwrap();
        let num_2: SnailfishNumber = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse().unwrap();
        let num_3: SnailfishNumber = "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".parse().unwrap();
        let num_4: SnailfishNumber = "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
            .parse()
            .unwrap();
        let num_5: SnailfishNumber = "[7,[5,[[3,8],[1,4]]]]".parse().unwrap();
        let num_6: SnailfishNumber = "[[2,[2,2]],[8,[8,1]]]".parse().unwrap();
        let num_7: SnailfishNumber = "[2,9]".parse().unwrap();
        let num_8: SnailfishNumber = "[1,[[[9,3],9],[[9,0],[0,7]]]]".parse().unwrap();
        let num_9: SnailfishNumber = "[[[5,[7,4]],7],1]".parse().unwrap();
        let num_10: SnailfishNumber = "[[[[4,2],2],6],[8,7]]".parse().unwrap();

        let sum = num_1 + num_2 + num_3 + num_4 + num_5 + num_6 + num_7 + num_8 + num_9 + num_10;

        assert_eq!(
            sum,
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<SnailfishNumber>()
                .unwrap()
        );
    }

    #[test]
    fn format() {
        let str_1 = "[[1,2],3]";
        let num_1: SnailfishNumber = str_1.parse().unwrap();
        let str_2 = format!("{}", num_1);
        assert_eq!(str_1, str_2);

        let str_1 = "[[1,9],[8,5]]";
        let num_1: SnailfishNumber = str_1.parse().unwrap();
        let str_2 = format!("{}", num_1);
        assert_eq!(str_1, str_2);

        let str_1 = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]";
        let num_1: SnailfishNumber = str_1.parse().unwrap();
        let str_2 = format!("{}", num_1);
        assert_eq!(str_1, str_2);
    }

    #[test]
    fn magnitude() {
        let num_1: SnailfishNumber = "[[1,2],[[3,4],5]]".parse().unwrap();
        assert_eq!(num_1.magnitude(), 143);

        let num_2: SnailfishNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!(num_2.magnitude(), 1384);

        let num_3: SnailfishNumber = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();
        assert_eq!(num_3.magnitude(), 3488);
    }
}
