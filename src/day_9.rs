use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct BasinMap {
    map: Vec<Vec<(u8, u64)>>,
}

impl BasinMap {
    fn get(&self, point: (usize, usize)) -> Option<&(u8, u64)> {
        self.map.get(point.0)?.get(point.1)
    }

    fn get_mut(&mut self, point: (usize, usize)) -> Option<&mut (u8, u64)> {
        self.map.get_mut(point.0)?.get_mut(point.1)
    }
}

impl Deref for BasinMap {
    type Target = Vec<Vec<(u8, u64)>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for BasinMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut part_1 = 0;

    let mut basin_map = BasinMap {
        map: lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|x| (x.to_digit(10).unwrap() as u8, 0))
                    .collect()
            })
            .collect(),
    };

    let mut num_basins = 0;
    let mut basin_sizes = vec![];

    for i in 0..basin_map.len() {
        for j in 0..basin_map[i].len() {
            let val = basin_map[i][j];
            let adj = generate_adj((i, j));

            let mut is_well = true;
            for a in adj {
                if let Some(adj_val) = basin_map.get(a) {
                    if val > *adj_val {
                        is_well = false;
                    }
                }
            }

            if is_well {
                part_1 += 1 + val.0 as u64;
                num_basins += 1;
                basin_map[i][j].1 = num_basins;
                basin_sizes.push(1);
            }
        }
    }

    for i in 0..basin_map.len() {
        for j in 0..basin_map[i].len() {
            if basin_map[i][j].0 == 9 {
                continue;
            }
            let mut flood = vec![];
            let mut edge = vec![(i, j)];

            'next_point: loop {
                let edge_point = edge.pop().unwrap();
                let edge_piece = basin_map.get(edge_point).unwrap();
                if edge_piece.1 != 0 {
                    break 'next_point;
                }

                let adj_points: Vec<(usize, usize)> = generate_adj(edge_point)
                    .drain(..)
                    .filter(|x| !flood.contains(x))
                    .filter(|x| matches!(basin_map.get(*x), Some(_)))
                    .filter(|x| edge_piece.0 > basin_map.get(*x).unwrap().0)
                    .collect();

                flood.push(edge_point);
                for a in adj_points {
                    let adj_val = *basin_map.get(a).unwrap();
                    if adj_val.1 != 0 {
                        flood.append(&mut edge);
                        for &point in &flood {
                            basin_map.get_mut(point).unwrap().1 = adj_val.1;
                        }
                        basin_sizes[(adj_val.1 - 1) as usize] += flood.len() as u64;
                        break 'next_point;
                    }
                    edge.push(a);
                }
            }
        }
    }

    basin_sizes.sort_unstable();
    let mut biggest_basins = basin_sizes.iter().rev();
    (
        part_1,
        biggest_basins.next().unwrap()
            * biggest_basins.next().unwrap()
            * biggest_basins.next().unwrap(),
    )
}

fn generate_adj(point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for i in -1_isize..=1 {
        result.push(((point.0 as isize + i) as usize, point.1));
    }
    for j in -1_isize..=1 {
        result.push((point.0, (point.1 as isize + j) as usize));
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![
            String::from("2199943210"),
            String::from("3987894921"),
            String::from("9856789892"),
            String::from("8767896789"),
            String::from("9899965678"),
        ];
        let result = run(&input);

        assert_eq!(result, (15, 1134));
    }
}
