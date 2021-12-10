use std::mem;

enum Orientation {
    Right(BracketType),
    Left(BracketType),
}

impl Orientation {
    fn inside(&self) -> &BracketType {
        match self {
            Orientation::Right(inside) | Orientation::Left(inside) => inside,
        }
    }
}

enum BracketType {
    Bracket,
    Paren,
    Brace,
    AngleBracket,
}

impl BracketType {
    fn from_char(s: char) -> Option<Orientation> {
        use BracketType::*;
        use Orientation::*;
        match s {
            '[' => Some(Left(Bracket)),
            ']' => Some(Right(Bracket)),
            '(' => Some(Left(Paren)),
            ')' => Some(Right(Paren)),
            '{' => Some(Left(Brace)),
            '}' => Some(Right(Brace)),
            '<' => Some(Left(AngleBracket)),
            '>' => Some(Right(AngleBracket)),
            _ => None,
        }
    }

    fn score_1(&self) -> u64 {
        match self {
            BracketType::Paren => 3,
            BracketType::Bracket => 57,
            BracketType::Brace => 1197,
            BracketType::AngleBracket => 25137,
        }
    }

    fn score_2(&self) -> u64 {
        match self {
            BracketType::Paren => 1,
            BracketType::Bracket => 2,
            BracketType::Brace => 3,
            BracketType::AngleBracket => 4,
        }
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut part_1 = 0;
    let mut part_2_scores = Vec::new();

    for line in lines {
        let mut stack = Vec::new();
        let mut valid = true;
        for c in line.chars() {
            let c = BracketType::from_char(c).unwrap();

            if let Orientation::Left(inside) = c {
                stack.push(inside);
            } else {
                match stack.pop() {
                    None => {
                        part_1 += c.inside().score_1();
                        valid = false;
                        break;
                    }
                    Some(lhs) => {
                        let inside_r = c.inside();
                        if mem::discriminant(&lhs) != mem::discriminant(&inside_r) {
                            part_1 += inside_r.score_1();
                            stack.push(lhs);
                            valid = false;
                            break;
                        }
                    }
                }
            }
        }
        if valid {
            let mut part_2_score = 0_u64;
            for remaining in stack.iter().rev() {
                part_2_score *= 5;
                part_2_score += remaining.score_2();
            }
            part_2_scores.push(part_2_score);
        }
    }

    let part_2_len = part_2_scores.len();
    part_2_scores.sort();
    let part_2 = part_2_scores[(part_2_len - 1) / 2];

    (part_1, part_2)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ];

        let result = run(&input);

        assert_eq!(result, (26397, 288957));
    }
}
