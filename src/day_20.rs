use std::collections::HashSet;

type Pixel = (i64, i64);
type Image = HashSet<Pixel>;
type Rule = Vec<bool>;

pub fn run(lines: &[String]) -> (u64, u64) {
    let rule: Rule = lines[0].chars().map(|c| c == '#').collect();
    let mut inverse_rule = Vec::with_capacity(rule.len());
    for i in 0..rule.len() {
        inverse_rule.push(!rule[!i & 0b111111111]);
    }

    let mut image = Image::new();

    for (y, line) in lines[2..].iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((x as i64, y as i64));
            }
        }
    }

    image = process_image(image, &rule);
    image = process_image(image, &inverse_rule);
    let part_1 = image.len() as u64;

    for _ in 0..24 {
        image = process_image(image, &rule);
        image = process_image(image, &inverse_rule);
    }
    let part_2 = image.len() as u64;

    (part_1, part_2)
}

fn process_image(image: Image, rule: &[bool]) -> Image {
    let mut result = Image::new();
    for pixel in &image {
        let adj = get_adj(*pixel);
        for a in adj {
            let rule_index = into_index(get_adj(a).map(|p| image.contains(&p)));
            if !rule[rule_index] {
                result.insert(a);
            }
        }
    }
    result
}

fn get_adj(point: Pixel) -> impl Iterator<Item = Pixel> {
    let mut result = Vec::new();
    for y in (point.1 - 1)..=(point.1 + 1) {
        for x in (point.0 - 1)..=(point.0 + 1) {
            result.push((x, y));
        }
    }
    result.into_iter()
}

fn into_index(grid: impl Iterator<Item = bool>) -> usize {
    let mut result = 0;
    for i in grid {
        result <<= 1;
        if i {
            result += 1;
        }
    }
    result
}
