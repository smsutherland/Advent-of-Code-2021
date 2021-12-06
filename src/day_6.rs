pub fn run(lines: &[String]) -> (u64, u64) {
    let line: Vec<u64> = lines[0].split(",").map(|x| x.parse().unwrap()).collect();
    let mut fish: Vec<u64> = (0..=8)
        .map(|x| line.iter().filter(|y| **y == x).count() as u64)
        .collect();

    let mut part_1 = 0;

    for i in 0..256 {
        let mut fish_iter = fish.drain(..);
        let new_fish = fish_iter.next().unwrap();
        fish = fish_iter.collect();
        fish.push(new_fish);
        fish[6] += new_fish;

        if i == 79 {
            part_1 = fish.iter().sum::<u64>();
        }

        // let mut new_fish = 0;
        // let mut fish_counter = |fish_age: u64|{
        //     let mut fish_age = fish_age;
        //     if fish_age == 0{
        //         fish_age = 6;
        //         new_fish += 1;
        //     }
        //     else{
        //         fish_age -= 1;
        //     }

        //     fish_age
        // };
        // fish = fish.iter().map(|x| fish_counter(*x)).collect();
        // for _ in 0..new_fish{
        //     fish.push(8);
        // }
    }
    (part_1, fish.iter().sum::<u64>())
}
