use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
struct Rotation([[i8; 3]; 3]);

const ALL_ROTATIONS: [Rotation; 24] = [
    Rotation([[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
    Rotation([[1, 0, 0], [0, -1, 0], [0, 0, -1]]),
    Rotation([[1, 0, 0], [0, 0, 1], [0, -1, 0]]),
    Rotation([[1, 0, 0], [0, 0, -1], [0, 1, 0]]),
    Rotation([[-1, 0, 0], [0, 1, 0], [0, 0, -1]]),
    Rotation([[-1, 0, 0], [0, -1, 0], [0, 0, 1]]),
    Rotation([[-1, 0, 0], [0, 0, 1], [0, 1, 0]]),
    Rotation([[-1, 0, 0], [0, 0, -1], [0, -1, 0]]),
    Rotation([[0, 1, 0], [1, 0, 0], [0, 0, -1]]),
    Rotation([[0, 1, 0], [-1, 0, 0], [0, 0, 1]]),
    Rotation([[0, 1, 0], [0, 0, 1], [1, 0, 0]]),
    Rotation([[0, 1, 0], [0, 0, -1], [-1, 0, 0]]),
    Rotation([[0, -1, 0], [1, 0, 0], [0, 0, 1]]),
    Rotation([[0, -1, 0], [-1, 0, 0], [0, 0, -1]]),
    Rotation([[0, -1, 0], [0, 0, 1], [-1, 0, 0]]),
    Rotation([[0, -1, 0], [0, 0, -1], [1, 0, 0]]),
    Rotation([[0, 0, 1], [1, 0, 0], [0, 1, 0]]),
    Rotation([[0, 0, 1], [-1, 0, 0], [0, -1, 0]]),
    Rotation([[0, 0, 1], [0, 1, 0], [-1, 0, 0]]),
    Rotation([[0, 0, 1], [0, -1, 0], [1, 0, 0]]),
    Rotation([[0, 0, -1], [1, 0, 0], [0, -1, 0]]),
    Rotation([[0, 0, -1], [-1, 0, 0], [0, 1, 0]]),
    Rotation([[0, 0, -1], [0, 1, 0], [1, 0, 0]]),
    Rotation([[0, 0, -1], [0, -1, 0], [-1, 0, 0]]),
];

impl Mul<Point> for Rotation {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point(
            self.0[0]
                .iter()
                .zip([rhs.0, rhs.1, rhs.2])
                .map(|x| x.1 * (*x.0 as i64))
                .sum(),
            self.0[1]
                .iter()
                .zip([rhs.0, rhs.1, rhs.2])
                .map(|x| x.1 * (*x.0 as i64))
                .sum(),
            self.0[2]
                .iter()
                .zip([rhs.0, rhs.1, rhs.2])
                .map(|x| x.1 * (*x.0 as i64))
                .sum(),
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point(i64, i64, i64);

impl Point {
    fn manhattan(&self) -> u64 {
        (self.0.abs() + self.1.abs() + self.2.abs()) as u64
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Clone, Debug)]
struct Scanner {
    beacon_positions: Vec<Point>,
    beacon_relative_positions: Vec<Vec<Point>>,
    scanner_positions: Vec<Point>,
    scanner_relative_positions: Vec<Vec<Point>>,
}

impl Scanner {
    fn new() -> Self {
        let mut result = Scanner {
            beacon_positions: Vec::new(),
            beacon_relative_positions: Vec::new(),
            scanner_positions: Vec::new(),
            scanner_relative_positions: Vec::new(),
        };
        result.add_scanner(Point(0, 0, 0));
        result
    }

    fn add_beacon(&mut self, p: Point) {
        let mut relative_positions = Vec::with_capacity(self.beacon_positions.len());
        for (i, b) in self.beacon_positions.iter().enumerate() {
            relative_positions.push(*b - p);
            self.beacon_relative_positions[i].push(p - *b);
        }
        self.beacon_positions.push(p);
        self.beacon_relative_positions.push(relative_positions);
    }

    fn add_scanner(&mut self, p: Point) {
        let mut relative_positions = Vec::with_capacity(self.scanner_positions.len());
        for (i, b) in self.scanner_positions.iter().enumerate() {
            relative_positions.push(*b - p);
            self.scanner_relative_positions[i].push(p - *b);
        }
        self.scanner_positions.push(p);
        self.scanner_relative_positions.push(relative_positions);
    }

    // Tries to align scanner_2 with scanner_1
    // If they align, returns Some((Point, Rotation)) containing the position and rotation of scanner_2 relative to scanner_1
    fn align(scanner_1: &Self, scanner_2: &Self) -> Option<(Point, Rotation)> {
        for (i, p1) in scanner_1.beacon_relative_positions.iter().enumerate() {
            for (j, p2) in scanner_2.beacon_relative_positions.iter().enumerate() {
                for r in ALL_ROTATIONS {
                    let p2: Vec<Point> = p2.iter().map(|p| r * *p).collect();
                    if Self::check_alignment(p1, &p2) {
                        let p1 = scanner_1.beacon_positions[i];
                        let p2 = r * scanner_2.beacon_positions[j];

                        return Some((p1 - p2, r));
                    }
                }
            }
        }
        None
    }

    fn check_alignment(relative_positions_1: &[Point], relative_positions_2: &[Point]) -> bool {
        // let matches: Vec<&Point> = relative_positions_1.iter().filter(|x| relative_positions_2.contains(x)).collect();
        // matches.len() >= 12
        relative_positions_1
            .iter()
            .filter(|x| relative_positions_2.contains(x))
            .count()
            >= 11 // 12th is the 0,0,0 one
    }

    fn combine_scanners(mut s1: Self, s2: Self, displacement: Point, rotation: Rotation) -> Self {
        for p in s2.beacon_positions {
            let p = rotation * p + displacement;
            if !s1.beacon_positions.contains(&p) {
                s1.add_beacon(p);
            }
        }
        for s in s2.scanner_positions {
            let s = rotation * s + displacement;
            s1.add_scanner(s);
        }
        s1
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut scanners = Vec::new();
    let mut current_scanner = Scanner::new();
    for line in lines {
        if line.starts_with("---") {
            continue;
        }
        if line.is_empty() {
            scanners.push(current_scanner);
            current_scanner = Scanner::new();
            continue;
        }

        let mut point_coords = line.split(',');
        let point = Point(
            point_coords.next().unwrap().parse().unwrap(),
            point_coords.next().unwrap().parse().unwrap(),
            point_coords.next().unwrap().parse().unwrap(),
        );
        current_scanner.add_beacon(point);
    }
    scanners.push(current_scanner);

    'main_loop: while scanners.len() > 1 {
        print!("{} left: ", scanners.len());
        for (i, s1) in scanners.iter().enumerate() {
            for (j, s2) in scanners.iter().enumerate() {
                if i == j {
                    continue;
                }

                if let Some((displacement, rotation)) = Scanner::align(s1, s2) {
                    println!("Combining scanners {} and {}.", i, j);
                    let s1 = s1.to_owned();
                    let s2 = s2.to_owned();
                    if i > j {
                        scanners.swap_remove(i);
                        scanners.swap_remove(j);
                    } else {
                        scanners.swap_remove(j);
                        scanners.swap_remove(i);
                    };
                    // scanners.push(Scanner::combine_scanners(s1, s2, displacement, rotation));
                    scanners.insert(0, Scanner::combine_scanners(s1, s2, displacement, rotation));
                    continue 'main_loop;
                }
            }
        }
        println!("._.");
        unreachable!();
    }

    let main_scanner = scanners.pop().unwrap();
    drop(scanners);

    let part_1 = main_scanner.beacon_positions.len() as u64;

    let mut part_2 = 0;
    for relative_set in main_scanner.scanner_relative_positions {
        for single_distance in &relative_set {
            let dist = single_distance.manhattan();
            if dist > part_2 {
                part_2 = dist;
            }
        }
    }

    (part_1, part_2)
}
