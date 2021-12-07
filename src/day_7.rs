pub fn run(lines: &[String]) -> (u64, u64) {
    // ((0..=(*lines[0].split(",").map(|x|x.parse().unwrap()).collect::<Vec<i64>>().iter().max().unwrap())).map(|i|lines[0].split(",").map(|x|x.parse().unwrap()).collect::<Vec<i64>>().iter().map(|x|(i-x).abs()).sum::<i64>()).min().unwrap() as u64,(0..=(*lines[0].split(",").map(|x|x.parse().unwrap()).collect::<Vec<i64>>().iter().max().unwrap())).map(|i|lines[0].split(",").map(|x|x.parse().unwrap()).collect::<Vec<i64>>().iter().map(|x|(i-x).abs()*((i-x).abs()+1)/2).sum::<i64>()).min().unwrap() as u64)

    let nums: Vec<i64> = lines[0].split(",").map(|x| x.parse().unwrap()).collect();
    (
        (0..=(*nums.iter().max().unwrap()))
            .map(|i| nums.iter().map(|x| (i - x).abs()).sum::<i64>())
            .min()
            .unwrap() as u64,
        (0..=(*nums.iter().max().unwrap()))
            .map(|i| {
                nums.iter()
                    .map(|x| (i - x).abs() * ((i - x).abs() + 1) / 2)
                    .sum::<i64>()
            })
            .min()
            .unwrap() as u64,
    )
}
