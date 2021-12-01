use itertools::izip;

pub fn day_1(lines: &[String]) -> (u32, u32){
    let depths = lines.iter().map(|x| x.parse::<u32>().unwrap());
    let part_1 = depths.clone()
            .zip(depths.clone().skip(1))
            .map(|x| x.0 < x.1)
            .filter(|x| *x == true)
            .count()
            as u32;

    let sums_of_3 = izip!(depths.clone(), depths.clone().skip(1), depths.clone().skip(2)).map(|x| x.0 + x.1 + x.2);
    let part_2 = sums_of_3.clone()
            .zip(sums_of_3.clone().skip(1))
            .map(|x| x.0 < x.1)
            .filter(|x| *x == true)
            .count()
            as u32;
        

    (part_1, part_2)
}