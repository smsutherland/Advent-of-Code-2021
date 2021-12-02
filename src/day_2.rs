pub fn run(lines: &[String]) -> (u32, u32) {
    let mut x = 0;
    let mut d = 0;

    let mut aim = 0;
    let mut d_2 = 0;
    for line in lines {
        let mut line_split = line.split(" ");
        let com = line_split.next().unwrap();
        let val: u32 = line_split.next().unwrap().parse().unwrap();

        match com {
            "forward" => {
                x += val;
                d_2 += aim * val;
            }
            "down" => {
                d += val;
                aim += val;
            }
            "up" => {
                d -= val;
                aim -= val;
            }
            _ => (),
        }
    }

    (x * d, x * d_2)
}
