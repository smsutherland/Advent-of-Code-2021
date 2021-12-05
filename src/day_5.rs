use std::collections::HashMap;

pub fn run(lines: &[String]) -> (u32, u32) {
    let mut board_1: HashMap<(i32, i32), u32> = HashMap::new();
    let mut board_2: HashMap<(i32, i32), u32> = HashMap::new();

    for line in lines {
        // let bits = common::deformat_str("{},{} -> {},{}}", line).unwrap(); // NOTE: Fix deformat_str

        let mut points = line.split(" -> ");
        let mut point0 = points.next().unwrap().split(",");
        let mut point1 = points.next().unwrap().split(",");
        let x0: i32 = point0.next().unwrap().parse().unwrap();
        let y0: i32 = point0.next().unwrap().parse().unwrap();
        let x1: i32 = point1.next().unwrap().parse().unwrap();
        let y1: i32 = point1.next().unwrap().parse().unwrap();

        if x0 == x1 {
            if y0 < y1 {
                for y in y0..=y1 {
                    *board_1.entry((x0, y)).or_insert(0) += 1;
                    *board_2.entry((x0, y)).or_insert(0) += 1;
                }
            } else {
                for y in y1..=y0 {
                    *board_1.entry((x0, y)).or_insert(0) += 1;
                    *board_2.entry((x0, y)).or_insert(0) += 1;
                }
            }
        } else if y0 == y1 {
            if x0 < x1 {
                for x in x0..=x1 {
                    *board_1.entry((x, y0)).or_insert(0) += 1;
                    *board_2.entry((x, y0)).or_insert(0) += 1;
                }
            } else {
                for x in x1..=x0 {
                    *board_1.entry((x, y0)).or_insert(0) += 1;
                    *board_2.entry((x, y0)).or_insert(0) += 1;
                }
            }
        } else if y1 - y0 == x1 - x0 {
            let start_x = if x0 < x1 { x0 } else { x1 };
            let start_y = if y0 < y1 { y0 } else { y1 };

            let mut diff = y1 - y0;
            if diff < 0 {
                diff = -diff;
            }

            for xy in 0..=diff {
                *board_2.entry((start_x + xy, start_y + xy)).or_insert(0) += 1;
            }
        } else if y1 - y0 == x0 - x1 {
            let start_x = if x0 < x1 { x0 } else { x1 };
            let start_y = if y0 < y1 { y1 } else { y0 };

            let mut diff = y1 - y0;
            if diff < 0 {
                diff = -diff;
            }
            for xy in 0..=diff {
                *board_2.entry((start_x + xy, start_y - xy)).or_insert(0) += 1;
            }
        }
    }

    let part_1 = board_1.iter().filter(|x| *x.1 > 1).count() as u32;
    let part_2 = board_2.iter().filter(|x| *x.1 > 1).count() as u32;

    (part_1, part_2)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            String::from("0,0 -> 8,8"),
            String::from("5,5 -> 8,2"),
        ];

        assert_eq!(run(&input), (5, 12));
    }
}
