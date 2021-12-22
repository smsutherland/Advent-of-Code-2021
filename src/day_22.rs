use crate::common;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type Point = (i64, i64, i64);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cubiod(
    RangeInclusive<i64>,
    RangeInclusive<i64>,
    RangeInclusive<i64>,
);

impl Cubiod {
    fn from_bounds(
        x_min: i64,
        x_max: i64,
        y_min: i64,
        y_max: i64,
        z_min: i64,
        z_max: i64,
    ) -> Cubiod {
        Cubiod(x_min..=x_max, y_min..=y_max, z_min..=z_max)
    }

    fn from_str(line: &str) -> Cubiod {
        let mut dimensions = line.split(",");
        let x = common::deformat_str("{}={}..{}", dimensions.next().unwrap()).unwrap();
        let y = common::deformat_str("{}={}..{}", dimensions.next().unwrap()).unwrap();
        let z = common::deformat_str("{}={}..{}", dimensions.next().unwrap()).unwrap();

        Cubiod::from_bounds(
            x[1].parse().unwrap(),
            x[2].parse().unwrap(),
            y[1].parse().unwrap(),
            y[2].parse().unwrap(),
            z[1].parse().unwrap(),
            z[2].parse().unwrap(),
        )
    }

    fn intersect(&self, other: &Cubiod) -> Cubiod {
        let x_min = *self.0.start().max(other.0.start());
        let x_max = *self.0.end().min(other.0.end());
        let y_min = *self.1.start().max(other.1.start());
        let y_max = *self.1.end().min(other.1.end());
        let z_min = *self.2.start().max(other.2.start());
        let z_max = *self.2.end().min(other.2.end());
        Cubiod::from_bounds(x_min, x_max, y_min, y_max, z_min, z_max)
    }

    fn difference(&self, other: &Cubiod) -> Vec<Cubiod> {
        let intersection = self.intersect(other);
        if intersection.is_empty() {
            return vec![self.clone()];
        }

        let mut result = Vec::new();

        // right
        result.push(Cubiod::from_bounds(
            *intersection.0.end() + 1,
            *self.0.end(),
            *self.1.start(),
            *self.1.end(),
            *self.2.start(),
            *self.2.end(),
        ));
        // left
        result.push(Cubiod::from_bounds(
            *self.0.start(),
            *intersection.0.start() - 1,
            *self.1.start(),
            *self.1.end(),
            *self.2.start(),
            *self.2.end(),
        ));

        // top
        result.push(Cubiod::from_bounds(
            *intersection.0.start(),
            *intersection.0.end(),
            *intersection.1.end() + 1,
            *self.1.end(),
            *self.2.start(),
            *self.2.end(),
        ));
        // bottom
        result.push(Cubiod::from_bounds(
            *intersection.0.start(),
            *intersection.0.end(),
            *self.1.start(),
            *intersection.1.start() - 1,
            *self.2.start(),
            *self.2.end(),
        ));

        // front
        result.push(Cubiod::from_bounds(
            *intersection.0.start(),
            *intersection.0.end(),
            *intersection.1.start(),
            *intersection.1.end(),
            *intersection.2.end() + 1,
            *self.2.end(),
        ));
        // back
        result.push(Cubiod::from_bounds(
            *intersection.0.start(),
            *intersection.0.end(),
            *intersection.1.start(),
            *intersection.1.end(),
            *self.2.start(),
            *intersection.2.start() - 1,
        ));

        result = result.into_iter().filter(|x| !x.is_empty()).collect();
        result
    }

    fn volume(&self) -> u64 {
        if !self.0.is_empty() && !self.1.is_empty() && !self.2.is_empty() {
            ((self.0.end() - self.0.start() + 1)
                * (self.1.end() - self.1.start() + 1)
                * (self.2.end() - self.2.start() + 1)) as u64
        } else {
            0
        }
    }

    fn is_empty(&self) -> bool {
        self.volume() == 0
    }

    fn make_set(&self) -> HashSet<Point> {
        let mut set = HashSet::new();
        for x in self.0.clone() {
            for y in self.1.clone() {
                for z in self.2.clone() {
                    set.insert((x, y, z));
                }
            }
        }
        set
    }
}

struct CubiodSet {
    components: Vec<Cubiod>,
}

impl CubiodSet {
    fn new() -> CubiodSet {
        CubiodSet {
            components: Vec::new(),
        }
    }

    fn union(&mut self, cubiod: Cubiod) {
        let mut added_cubiods = vec![cubiod];
        'main_loop: while let Some(new_cubiod) = added_cubiods.pop() {
            for component in &self.components {
                let mut parts = new_cubiod.difference(component);
                if parts.len() > 1 || parts.get(0) != Some(&new_cubiod) {
                    // dbg!(parts.len());
                    // dbg!(&parts[0]);
                    // dbg!(new_cubiod);
                    // dbg!(component);
                    added_cubiods.append(&mut parts);
                    continue 'main_loop;
                }
            }
            self.components.push(new_cubiod);
        }
    }

    fn difference(&mut self, cubiod: Cubiod) {
        let mut new_components = Vec::new();
        while let Some(component) = self.components.pop() {
            let mut parts = component.difference(&cubiod);
            new_components.append(&mut parts);
        }
        self.components = new_components;
    }

    fn len(&self) -> u64 {
        let mut total = 0;
        for component in &self.components {
            total += component.volume();
        }
        total
    }

    fn reduce(&mut self) {
        'main_loop: loop {
            for (i, component_1) in self.components.iter().enumerate() {
                for (j, component_2) in self.components.iter().enumerate() {
                    if i == j {
                        continue;
                    }

                    // check x direction
                    if component_1.1 == component_2.1 && component_1.2 == component_2.2 {
                        if component_1.0.end() + 1 == *component_2.0.start()
                            || *component_1.0.start() == component_2.0.end() + 1
                        {
                            let new_component = Cubiod::from_bounds(
                                *component_1.0.start().min(component_2.0.start()),
                                *component_1.0.end().max(component_2.0.end()),
                                *component_1.1.start(),
                                *component_1.1.end(),
                                *component_1.2.start(),
                                *component_1.2.end(),
                            );
                            if i > j {
                                self.components.swap_remove(i);
                                self.components.swap_remove(j);
                            } else {
                                self.components.swap_remove(j);
                                self.components.swap_remove(i);
                            };
                            self.components.push(new_component);
                            continue 'main_loop;
                        }
                    }

                    // check y direction
                    if component_1.0 == component_2.0 && component_1.2 == component_2.2 {
                        if component_1.1.end() + 1 == *component_2.1.start()
                            || *component_1.1.start() == component_2.1.end() + 1
                        {
                            let new_component = Cubiod::from_bounds(
                                *component_1.0.start(),
                                *component_1.0.end(),
                                *component_1.1.start().min(component_2.1.start()),
                                *component_1.1.end().max(component_2.1.end()),
                                *component_1.2.start(),
                                *component_1.2.end(),
                            );
                            if i > j {
                                self.components.swap_remove(i);
                                self.components.swap_remove(j);
                            } else {
                                self.components.swap_remove(j);
                                self.components.swap_remove(i);
                            };
                            self.components.push(new_component);
                            continue 'main_loop;
                        }
                    }

                    // check z direction
                    if component_1.0 == component_2.0 && component_1.1 == component_2.1 {
                        if component_1.2.end() + 1 == *component_2.2.start()
                            || *component_1.2.start() == component_2.2.end() + 1
                        {
                            let new_component = Cubiod::from_bounds(
                                *component_1.0.start(),
                                *component_1.0.end(),
                                *component_1.1.start(),
                                *component_1.1.end(),
                                *component_1.2.start().min(component_2.2.start()),
                                *component_1.2.end().max(component_2.2.end()),
                            );
                            if i > j {
                                self.components.swap_remove(i);
                                self.components.swap_remove(j);
                            } else {
                                self.components.swap_remove(j);
                                self.components.swap_remove(i);
                            };
                            self.components.push(new_component);
                            continue 'main_loop;
                        }
                    }
                }
            }
            break;
        }
    }
}

const PART_1_BOUNDARY: Cubiod = Cubiod(-50..=50, -50..=50, -50..=50);

pub fn run(lines: &[String]) -> (u64, u64) {
    let mut lit_1 = HashSet::new();
    let mut lit_2 = CubiodSet::new();
    for line in lines {
        let bits = common::deformat_str("{} {}", line).unwrap();
        let on = bits[0] == "on";
        let cubiod = Cubiod::from_str(&bits[1]);

        // let restricted_cubiod = restrict_cubiod(&cubiod);
        let restricted_cubiod = PART_1_BOUNDARY.intersect(&cubiod);
        let this_set = restricted_cubiod.make_set();

        if on {
            lit_1 = lit_1.union(&this_set).map(|x| x.to_owned()).collect();
            lit_2.union(cubiod);
            lit_2.reduce();
        } else {
            lit_1 = lit_1.difference(&this_set).map(|x| x.to_owned()).collect();
            lit_2.difference(cubiod);
            lit_2.reduce();
        }
    }
    let part_1 = lit_1.len() as u64;
    let part_2 = lit_2.len();

    (part_1, part_2)
}
