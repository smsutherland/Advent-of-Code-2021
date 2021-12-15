use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug)]
struct RiskMap {
    map: Vec<Vec<u8>>,
}

impl RiskMap {
    fn get(&self, point: (usize, usize)) -> Option<u8> {
        let size = (self.len(), self[0].len());
        let depth = (point.0 / size.0 + point.1 / size.1) as u8;
        let mapped_point = (point.0 % size.0, point.1 % size.1);

        Some((self.map.get(mapped_point.0)?.get(mapped_point.1)? + depth - 1) % 9 + 1)
    }

    // fn get_mut(&mut self, point: (usize, usize)) -> Option<&mut u8> {
    //     self.map.get_mut(point.0)?.get_mut(point.1)
    // }
}

impl Deref for RiskMap {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let risk_map = RiskMap {
        map: lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    };

    let mut calculator = RiskCalculator::new(risk_map);

    let part_1 = calculator.min_risk();
    calculator.reset_cache();
    let part_2 = calculator.min_risk_2();

    (part_1, part_2)
}

struct RiskCalculator {
    risk_map: RiskMap,
    cache: HashMap<(usize, usize), u64>,
    size: (usize, usize),
    stack: Vec<(usize, usize)>, // only used for part 2
}

impl RiskCalculator {
    fn new(risk_map: RiskMap) -> RiskCalculator {
        RiskCalculator {
            size: (risk_map.len(), risk_map[0].len()),
            risk_map,
            cache: HashMap::new(),
            stack: Vec::new(),
        }
    }

    fn reset_cache(&mut self) {
        self.cache = HashMap::new();
    }

    fn min_risk(&mut self) -> u64 {
        self.reset_cache();
        self.min_risk_maker((0, 0), (self.size.0 - 1, self.size.1 - 1), &mut vec![])
            - self.risk_map.get((0, 0)).unwrap() as u64
    }

    fn min_risk_2(&mut self) -> u64 {
        self.reset_cache();
        self.stack.push((5 * self.size.0 - 1, 5 * self.size.1 - 1));
        while let Some(p) = self.stack.pop(){
            self.min_risk_maker_2(p);
        }
        println!("{:?}", self.cache.get(&(49, 49)));
        *self.cache.get(&(0, 0)).unwrap() - self.risk_map.get((0, 0)).unwrap() as u64
    }

    fn min_risk_maker(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        mut visited: &mut Vec<(usize, usize)>,
    ) -> u64 {
        if let Some(val) = self.cache.get(&start) {
            return *val;
        }
        if visited.contains(&start) {
            return u64::MAX;
        }

        let this_point = self.risk_map.get(start).unwrap();
        if start == end {
            return this_point as u64;
        }

        visited.push(start);
        let path_1 = if start.0 < end.0 {
            self.min_risk_maker((start.0 + 1, start.1), end, &mut visited)
        } else {
            u64::MAX
        };
        let path_2 = if start.1 < end.1 {
            self.min_risk_maker((start.0, start.1 + 1), end, &mut visited)
        } else {
            u64::MAX
        };
        // let path_3 = if start.0 > 0{
        //     self.min_risk_maker((start.0 - 1, start.1), end, &mut visited)
        // }
        // else{
        //     u64::MAX
        // };
        // let path_4 = if start.1 > 0{
        //     self.min_risk_maker((start.0, start.1 - 1), end, &mut visited)
        // }
        // else{
        //     u64::MAX
        // };
        visited.pop();

        // let result = vec![path_1, path_2, path_3, path_4].iter().min().unwrap() + this_point as u64;
        let result = path_1.min(path_2) + this_point as u64;
        self.cache.insert(start, result);
        result
    }

    fn min_risk_maker_2(&mut self, start: (usize, usize)) -> u64 {
        // print_cache(&self.cache, (5*self.size.0, 5*self.size.1));

        let mut valid_adj = vec![];
        if start.1 > 0 {
            valid_adj.push((start.0, start.1 - 1));
        }
        if start.0 > 0 {
            valid_adj.push((start.0 - 1, start.1));
        }
        if start.1 < 5 * self.size.1 - 1 {
            valid_adj.push((start.0, start.1 + 1));
        }
        if start.0 < 5 * self.size.0 - 1 {
            valid_adj.push((start.0 + 1, start.1));
        }

        let mut paths = vec![];
        for v in &valid_adj {
            if let Some(val) = self.cache.get(&v) {
                paths.push(*val);
            }
        }
        
        let this_point = self.risk_map.get(start).unwrap();
        let result = paths.into_iter().min().unwrap_or(0) + this_point as u64;



        let prev_val = *self.cache.get(&start).unwrap_or(&u64::MAX);
        self.cache.insert(start, result.min(prev_val)).unwrap_or(u64::MAX);

        if prev_val > result {
            for v in &valid_adj {
                self.stack.push(*v);
            }
        }
        result
    }
}
// 2196 2948 2954

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![
            String::from("1163751742"),
            String::from("1381373672"),
            String::from("2136511328"),
            String::from("3694931569"),
            String::from("7463417111"),
            String::from("1319128137"),
            String::from("1359912421"),
            String::from("3125421639"),
            String::from("1293138521"),
            String::from("2311944581"),
        ];

        let result = run(&input);

        assert_eq!(result, (40, 315));
    }
}
