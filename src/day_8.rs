use std::collections::{HashMap, HashSet};

//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  ggg

pub fn run(lines: &[String]) -> (u64, u64) {
    let zero:  HashSet<_> = vec!['a', 'b', 'c', 'e', 'f', 'g'].drain(..).collect();
    let one:   HashSet<_> = vec!['c', 'f'].drain(..).collect();
    let two:   HashSet<_> = vec!['a', 'c', 'd', 'e', 'g'].drain(..).collect();
    let three: HashSet<_> = vec!['a', 'c', 'd', 'f', 'g'].drain(..).collect();
    let four:  HashSet<_> = vec!['b', 'c', 'd', 'f'].drain(..).collect();
    let five:  HashSet<_> = vec!['a', 'b', 'd', 'f', 'g'].drain(..).collect();
    let six:   HashSet<_> = vec!['a', 'b', 'd', 'e', 'f', 'g'].drain(..).collect();
    let seven: HashSet<_> = vec!['a', 'c', 'f'].drain(..).collect();
    let eight: HashSet<_> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'].drain(..).collect();
    let nine:  HashSet<_> = vec!['a', 'b', 'c', 'd', 'f', 'g'].drain(..).collect();
    let digits = vec![zero, one, two, three, four, five, six, seven, eight, nine];

    let mut part_1 = 0;
    let mut part_2 = 0;
    for line in lines {
        let mut parts = line.split(" | ");
        let input: Vec<&str> = parts.next().unwrap().split(" ").collect();
        let output = parts.next().unwrap().split(" ");

        part_1 = part_1
            + output
                .clone()
                .map(|x| x.len() as i64)
                .filter(|x| matches!(x, 2 | 3 | 4 | 7))
                .count() as u64;

        let unknown_digits: Vec<HashSet<char>> =
            input.iter().map(|x| x.chars().collect()).collect();

        let mut these_digits = vec![HashSet::new(); 10];
        let mut letter_map = HashMap::<char, char>::new();
        for i in &input {
            match i.len() {
                2 => {
                    for c in i.chars() {
                        these_digits[1].insert(c);
                    }
                }
                3 => {
                    for c in i.chars() {
                        these_digits[7].insert(c);
                    }
                }
                4 => {
                    for c in i.chars() {
                        these_digits[4].insert(c);
                    }
                }
                7 => {
                    for c in i.chars() {
                        these_digits[8].insert(c);
                    }
                }
                _ => (),
            }
        }
        letter_map.insert(
            'a',
            *(&these_digits[7] - &these_digits[1]).iter().next().unwrap(),
        );

        let (bot_side, nine_index) = find_one_more(
            &unknown_digits,
            these_digits[4].union(&these_digits[7]).cloned().collect(),
        );
        letter_map.insert('g', bot_side);
        these_digits[9] = these_digits[9]
            .union(&unknown_digits[nine_index])
            .cloned()
            .collect();

        letter_map.insert(
            'e',
            *(&these_digits[8] - &these_digits[9]).iter().next().unwrap(),
        );

        let mut seven_and_bot_and_bot_l = these_digits[7].clone();
        seven_and_bot_and_bot_l.insert(letter_map[&'g']);
        seven_and_bot_and_bot_l.insert(letter_map[&'e']);
        let (top_l_side, zero_index) = find_one_more(&unknown_digits, seven_and_bot_and_bot_l);
        letter_map.insert('b', top_l_side);
        these_digits[0] = these_digits[0]
            .union(&unknown_digits[zero_index])
            .cloned()
            .collect();

        let mut four_minus_one_minus_top_l = these_digits[4].clone();
        four_minus_one_minus_top_l = &four_minus_one_minus_top_l - &these_digits[1];
        four_minus_one_minus_top_l.remove(&letter_map[&'b']);
        letter_map.insert('d', *four_minus_one_minus_top_l.iter().next().unwrap());

        let mut top_plus_mid_plus_bot_plus_bot_l = HashSet::new();
        top_plus_mid_plus_bot_plus_bot_l.insert(letter_map[&'a']);
        top_plus_mid_plus_bot_plus_bot_l.insert(letter_map[&'d']);
        top_plus_mid_plus_bot_plus_bot_l.insert(letter_map[&'e']);
        top_plus_mid_plus_bot_plus_bot_l.insert(letter_map[&'g']);
        let (top_r_side, two_index) =
            find_one_more(&unknown_digits, top_plus_mid_plus_bot_plus_bot_l);
        letter_map.insert('c', top_r_side);
        these_digits[2] = these_digits[2]
            .union(&unknown_digits[two_index])
            .cloned()
            .collect();

        let mut bot_r = these_digits[8].clone();
        bot_r.remove(&letter_map[&'a']);
        bot_r.remove(&letter_map[&'b']);
        bot_r.remove(&letter_map[&'c']);
        bot_r.remove(&letter_map[&'d']);
        bot_r.remove(&letter_map[&'e']);
        bot_r.remove(&letter_map[&'g']);
        letter_map.insert('f', *bot_r.iter().next().unwrap());

        let mut inverted_letter_map = HashMap::new();
        for (key, val) in letter_map.iter(){
            inverted_letter_map.insert(*val, *key);
        }
        
        let mut unknown_num = 0;
        for digit in output {
            unknown_num *= 10;
            let digit_set: HashSet<char> = digit.chars().map(|x| inverted_letter_map[&x]).collect();

            for (i, num) in digits.iter().enumerate() {
                if *num == digit_set {
                    unknown_num += i;
                    break;
                }
            }
        }
        part_2 += unknown_num as u64;
    }

    (part_1, part_2)
}

fn find_one_more(unknown_digits: &Vec<HashSet<char>>, all_but_one: HashSet<char>) -> (char, usize) {
    for (i, digit) in unknown_digits.iter().enumerate() {
        if !all_but_one.is_subset(&digit) {
            continue;
        }
        let mut diff = digit.difference(&all_but_one);
        if let Some(n) = diff.next() {
            if let None = diff.next() {
                return (*n, i);
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"),
        ];

        let result = run(&input);

        assert_eq!(result, (26, 61229));
    }

    #[test]
    fn test_1(){
        let input = vec![String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")];
        let result = run(&input);

        assert_eq!(result, (0, 5353));

    }
}
