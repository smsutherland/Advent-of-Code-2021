use crate::common;
use std::collections::HashSet;

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut caves = Vec::new();
    let mut connections = Vec::new();
    let mut bigs = HashSet::new();
    for line in lines {
        let mut line_caves = common::deformat_str("{}-{}", line).unwrap();
        let mut line_caves_iter = line_caves.drain(..);
        let start = line_caves_iter.next().unwrap();
        let end = line_caves_iter.next().unwrap();

        let start_index = if !caves.contains(&start) {
            caves.push(start.clone());
            caves.len() - 1
        } else {
            caves.iter().position(|x| *x == start).unwrap()
        };
        let end_index = if !caves.contains(&end) {
            caves.push(end.clone());
            caves.len() - 1
        } else {
            caves.iter().position(|x| *x == end).unwrap()
        };

        if big(&start) {
            bigs.insert(start_index);
        }
        if big(&end) {
            bigs.insert(end_index);
        }

        connections.push((start_index, end_index));
    }
    let start_index = caves.iter().position(|x| *x == "start").unwrap();
    let end_index = caves.iter().position(|x| *x == "end").unwrap();
    drop(caves);

    let part_1 = num_paths_1(&connections, vec![], start_index, end_index, &bigs);
    let part_2 = num_paths_2(
        &connections,
        vec![],
        start_index,
        end_index,
        &bigs,
        false,
        start_index,
    );

    (part_1, part_2)
}

fn big(cave_name: &String) -> bool {
    *cave_name == cave_name.to_ascii_uppercase()
}

fn num_paths_1(
    cave_paths: &Vec<(usize, usize)>,
    mut visited: Vec<usize>,
    start: usize,
    end: usize,
    bigs: &HashSet<usize>,
) -> u64 {
    if start == end {
        return 1;
    }
    if !bigs.contains(&start) {
        visited.push(start);
    }
    let adj = get_adj(&cave_paths, start)
        .filter(|x| !visited.contains(x));
    let mut paths = 0;
    for next in adj {
        paths += num_paths_1(&cave_paths, visited.clone(), next, end, &bigs);
    }
    paths
}

fn num_paths_2(
    cave_paths: &Vec<(usize, usize)>,
    mut visited: Vec<usize>,
    start: usize,
    end: usize,
    bigs: &HashSet<usize>,
    visited_small: bool,
    absolute_start: usize,
) -> u64 {
    if start == end {
        return 1;
    }
    if !bigs.contains(&start) {
        visited.push(start);
    }
    let adj = get_adj(&cave_paths, start)
        .filter(|x| (!visited_small || !visited.contains(x)) && *x != absolute_start);
    let mut paths = 0;
    for next in adj {
        paths += num_paths_2(
            &cave_paths,
            visited.clone(),
            next,
            end,
            &bigs,
            visited_small || visited.contains(&next),
            absolute_start,
        );
    }
    paths
}

fn get_adj<'a>(cave_paths: &'a Vec<(usize, usize)>, cave_index: usize) -> impl Iterator<Item=usize> + 'a {
    cave_paths
        .iter()
        .filter(move |x| x.0 == cave_index || x.1 == cave_index)
        .map(move |x| if x.0 == cave_index { x.1 } else { x.0 })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![
            String::from("start-A"),
            String::from("start-b"),
            String::from("A-c"),
            String::from("A-b"),
            String::from("b-d"),
            String::from("A-end"),
            String::from("b-end"),
        ];

        let result = run(&input);

        assert_eq!(result, (10, 36));
    }

    #[test]
    fn example_2() {
        let input = vec![
            String::from("dc-end"),
            String::from("HN-start"),
            String::from("start-kj"),
            String::from("dc-start"),
            String::from("dc-HN"),
            String::from("LN-dc"),
            String::from("HN-end"),
            String::from("kj-sa"),
            String::from("kj-HN"),
            String::from("kj-dc"),
        ];

        let result = run(&input);

        assert_eq!(result.0, 19);
    }

    #[test]
    fn example_3() {
        let input = vec![
            String::from("fs-end"),
            String::from("he-DX"),
            String::from("fs-he"),
            String::from("start-DX"),
            String::from("pj-DX"),
            String::from("end-zg"),
            String::from("zg-sl"),
            String::from("zg-pj"),
            String::from("pj-he"),
            String::from("RW-he"),
            String::from("fs-DX"),
            String::from("pj-RW"),
            String::from("zg-RW"),
            String::from("start-pj"),
            String::from("he-WI"),
            String::from("zg-he"),
            String::from("pj-fs"),
            String::from("start-RW"),
        ];

        let result = run(&input);

        assert_eq!(result.0, 226);
    }
}
