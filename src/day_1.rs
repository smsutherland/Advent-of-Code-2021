pub fn run(lines: &[String]) -> (u64, u64) {
    let depths: Vec<u64> = lines.iter().map(|x| x.parse::<u64>().unwrap()).collect();
    let part_1 = depths
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count() as u64;

    let part_2 = 
        depths.windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count() as u64;

    (part_1, part_2)
}
