use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Cucumber {
    East,
    South,
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut cucumbers = HashMap::new();
    let size = (lines[0].len(), lines.len());

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => {
                    cucumbers.insert((x, y), Cucumber::East);
                }
                'v' => {
                    cucumbers.insert((x, y), Cucumber::South);
                }
                _ => {}
            }
        }
    }

    let mut prev_cuc = HashMap::new();
    let mut i = 0;
    while prev_cuc != cucumbers {
        prev_cuc = cucumbers.clone();
        cucumbers = step(cucumbers, size);
        i += 1;
    }

    (i, 0)
}

fn step(
    cucumbers: HashMap<(usize, usize), Cucumber>,
    size: (usize, usize),
) -> HashMap<(usize, usize), Cucumber> {
    let mut result = HashMap::new();
    for (pos, c) in &cucumbers {
        if let Cucumber::East = c {
            let pos_to_check = ((pos.0 + 1) % size.0, pos.1);
            if cucumbers.get(&pos_to_check).is_none() {
                result.insert(pos_to_check, Cucumber::East);
            } else {
                result.insert(*pos, Cucumber::East);
            }
        }
    }
    for (pos, c) in &cucumbers {
        if let Cucumber::South = c {
            let pos_to_check = (pos.0, (pos.1 + 1) % size.1);
            if !matches!(cucumbers.get(&pos_to_check), Some(&Cucumber::South))
                && !matches!(result.get(&pos_to_check), Some(&Cucumber::East))
            {
                result.insert(pos_to_check, Cucumber::South);
            } else {
                result.insert(*pos, Cucumber::South);
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![
            String::from("v...>>.vv>"),
            String::from(".vv>>.vv.."),
            String::from(">>.>v>...v"),
            String::from(">>v>>.>.v."),
            String::from("v>v.vv.v.."),
            String::from(">.>>..v..."),
            String::from(".vv..>.>v."),
            String::from("v.v..>>v.v"),
            String::from("....v..v.>"),
        ];

        let result = run(&input);

        assert_eq!(result, (58, 0));
    }
}
