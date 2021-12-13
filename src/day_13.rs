use crate::common;
use std::collections::HashSet;

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut parts = lines.split(|x| x == "");
    let points: HashSet<[u64; 2]> = parts
        .next()
        .unwrap()
        .iter()
        .map(|p| {
            let v = p.split(",").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
            [v[0], v[1]]
        })
        .collect();
    let folds: Vec<(char, u64)> = parts
        .next()
        .unwrap()
        .iter()
        .map(|x| {
            let v = common::deformat_str("fold along {}={}", x).unwrap();
            (v[0].chars().next().unwrap(), v[1].parse().unwrap())
        })
        .collect();

    let first_fold = folds[0];

    let mut new_points = HashSet::new();
    for point in points {
        if let Some(new_point) = reflect(point, first_fold.1, first_fold.0) {
            new_points.insert(new_point);
        }
    }
    let part_1 = new_points.len() as u64;

    for fold in folds.iter().skip(1) {
        let points = new_points.clone();
        new_points = HashSet::new();
        for point in points {
            if let Some(new_point) = reflect(point, fold.1, fold.0) {
                new_points.insert(new_point);
            }
        }
    }

    display(new_points);

    (part_1, 0)
}

fn reflect(point: [u64; 2], over: u64, axis: char) -> Option<[u64; 2]> {
    let axis = match axis {
        'x' => 0,
        'y' => 1,
        _ => unreachable!(),
    };

    let mut new_point = point.clone();
    // assert!(new_point[axis] < 2*over, "folding over too far");
    new_point[axis] = if new_point[axis] > over {
        if new_point[axis] > 2 * over {
            return None;
        }
        2 * over - new_point[axis]
    } else {
        new_point[axis]
    };
    Some(new_point)
}

fn display(points: HashSet<[u64; 2]>) {
    let max_x = points.iter().map(|x| x[0]).max().unwrap();
    let max_y = points.iter().map(|x| x[1]).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&[x, y]) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
