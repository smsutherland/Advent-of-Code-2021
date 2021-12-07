pub fn run(lines: &[String]) -> (u64, u64){
    let nums: Vec<i64> = lines[0].split(",").map(|x| x.parse().unwrap()).collect();

    let mut min_fuel = 1000000000000_u64;
    for i in 0..(*nums.iter().max().unwrap()){
        let fuel = nums.iter().map(|x| (i-x).abs()).sum::<i64>();
        if fuel < min_fuel as i64{
            min_fuel = fuel as u64;
        }
    }

    let mut min_fuel2 = 1000000000000_u64;
    for i in 0..(*nums.iter().max().unwrap()){
        let fuel = nums.iter().map(|x| (i-x).abs()*((i-x).abs()+1)/2).sum::<i64>();
        if fuel < min_fuel2 as i64{
            min_fuel2 = fuel as u64;
        }
    }


    (min_fuel, min_fuel2)
}