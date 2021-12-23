use crate::common;
use std::collections::HashMap;

pub fn run(lines: &[String]) -> (u64, u64) {
    let polymer = lines[0].clone();
    let mut insertions = HashMap::new();
    for line in lines.iter().skip(2) {
        let deformatted = common::deformat_str("{} -> {}", line).unwrap();
        insertions.insert(
            deformatted[0].clone(),
            deformatted[1].chars().next().unwrap(),
        );
    }

    let part_1 = do_it(&polymer, &insertions, 10);
    let part_2 = do_it(&polymer, &insertions, 40);

    (part_1, part_2)
}

fn do_it(polymer: &str, insertions: &HashMap<String, char>, i: u32) -> u64 {
    let mut pair_counts = HashMap::new();
    for pair in pairs(polymer) {
        *pair_counts.entry(pair.to_owned()).or_insert(0) += 1;
    }

    for _ in 0..i {
        let mut new_pair_counts = HashMap::new();
        for (pair, count) in pair_counts {
            let insert = insertions[pair.as_str()];
            let mut pair_1 = pair.clone();
            pair_1.pop();
            pair_1.push(insert);
            let mut pair_2 = pair.clone();
            pair_2.remove(0);
            pair_2.insert(0, insert);
            *new_pair_counts
                .entry(pair_1.as_str().to_owned())
                .or_insert(0) += count;
            *new_pair_counts
                .entry(pair_2.as_str().to_owned())
                .or_insert(0) += count;
        }
        pair_counts = new_pair_counts.clone();
    }

    let mut char_counts = HashMap::new();
    for (pair, count) in pair_counts {
        let mut chars = pair.chars();
        let char_1 = chars.next().unwrap();
        let char_2 = chars.next().unwrap();
        *char_counts.entry(char_1).or_insert(0) += count;
        *char_counts.entry(char_2).or_insert(0) += count;
    }
    *char_counts
        .entry(polymer.chars().next().unwrap())
        .or_insert(0) += 1;
    *char_counts
        .entry(polymer.chars().rev().next().unwrap())
        .or_insert(0) += 1;

    for val in char_counts.values_mut() {
        *val /= 2;
    }

    char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
}

fn pairs(line: &str) -> impl Iterator<Item = &str> {
    (0..line.len() - 1).map(|i| &line[i..i + 2])
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![
            String::from("NNCB"),
            String::from(""),
            String::from("CH -> B"),
            String::from("HH -> N"),
            String::from("CB -> H"),
            String::from("NH -> C"),
            String::from("HB -> C"),
            String::from("HC -> B"),
            String::from("HN -> C"),
            String::from("NN -> C"),
            String::from("BH -> H"),
            String::from("NC -> B"),
            String::from("NB -> B"),
            String::from("BN -> B"),
            String::from("BB -> N"),
            String::from("BC -> B"),
            String::from("CC -> N"),
            String::from("CN -> C"),
        ];

        let result = run(&input);

        assert_eq!(result, (1588, 2188189693529))
    }
}
