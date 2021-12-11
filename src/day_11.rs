use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct OctoMap {
    map: Vec<Vec<u8>>,
}

impl OctoMap {
    fn _get(&self, point: (usize, usize)) -> Option<&u8> {
        self.map.get(point.0)?.get(point.1)
    }

    fn get_mut(&mut self, point: (usize, usize)) -> Option<&mut u8> {
        self.map.get_mut(point.0)?.get_mut(point.1)
    }

    fn inc_point(&mut self, point: (usize, usize)) {
        if let Some(octo) = self.get_mut(point) {
            *octo += 1;
            if *octo == 10 {
                let adj = generate_adj(point);
                for adj_point in adj {
                    self.inc_point(adj_point);
                }
            }
        }
    }

    fn count_flashes(&mut self) -> u64 {
        self.map = self
            .map
            .iter()
            .map(|row| row.iter().map(|x| if *x >= 10 { 0 } else { *x }).collect())
            .collect();

        self.map
            .iter()
            .map(|row| row.iter().filter(|x| **x == 0).count() as u64)
            .sum()
    }
}

impl Deref for OctoMap {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for OctoMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut part_1 = 0;

    let mut octo_map = OctoMap {
        map: lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    };

    let mut i = 0;
    loop {
        for y in 0..10 {
            for x in 0..10 {
                octo_map.inc_point((y, x));
            }
        }
        let flashes = octo_map.count_flashes();
        if i < 100 {
            part_1 += flashes;
        }
        i += 1;
        if flashes == 100 {
            break;
        }
    }

    (part_1, i)
}

fn generate_adj(point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for i in -1_isize..=1 {
        for j in -1_isize..=1 {
            result.push((
                (point.0 as isize + i) as usize,
                (point.1 as isize + j) as usize,
            ));
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
            String::from("5483143223"),
            String::from("2745854711"),
            String::from("5264556173"),
            String::from("6141336146"),
            String::from("6357385478"),
            String::from("4167524645"),
            String::from("2176841721"),
            String::from("6882881134"),
            String::from("4846848554"),
            String::from("5283751526"),
        ];
        let result = run(&input);

        assert_eq!(result, (1656, 195));
    }
}
