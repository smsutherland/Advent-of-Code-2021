use itertools::izip;

pub fn run(lines: &[String]) -> (u64, u64) {
    let depths = lines.iter().map(|x| x.parse::<u64>().unwrap());
    let part_1 = depths
        .clone()
        .zip(depths.clone().skip(1))
        .map(|x| x.0 < x.1)
        .filter(|x| *x)
        .count() as u64;

    let sums_of_3 = izip!(
        depths.clone(),
        depths.clone().skip(1),
        depths.clone().skip(2)
    )
    .map(|x| x.0 + x.1 + x.2);
    let part_2 = sums_of_3
        .clone()
        .zip(sums_of_3.clone().skip(1))
        .map(|x| x.0 < x.1)
        .filter(|x| *x)
        .count() as u64;

    (part_1, part_2)
}
