use crate::common;

pub fn run(lines: &[String]) -> (u64, u64){
    let mut area = common::deformat_str("target area: x={}..{}, y={}..{}", &lines[0]).unwrap().into_iter();
    let x_min: i32 = area.next().unwrap().parse().unwrap();
    let x_max: i32 = area.next().unwrap().parse().unwrap();
    let y_min: i32 = area.next().unwrap().parse().unwrap();
    let y_max: i32 = area.next().unwrap().parse().unwrap();
    let x_range = x_min..=x_max;
    let y_range = y_min..=y_max;

    let max_v_y = -y_min - 1;
    let min_v_y = -max_v_y - 1;
    let max_h = (max_v_y*(max_v_y + 1))/2;

    let max_v_x = x_max;
    let min_v_x = get_min_v_x(x_min);

    let mut total = 0;
    for v_y in min_v_y..=max_v_y{
        for v_x in min_v_x..=max_v_x{
            if reaches_target(v_x, v_y, &x_range, &y_range){
                total += 1;
            }
        }
    }

    (max_h as u64, total)
}

fn get_min_v_x(x_min: i32) -> i32{
    let mut v_x = 0;
    let mut d = 0;
    loop{
        if d >= x_min{
            return v_x;
        }

        v_x += 1;
        d += v_x;
    }
}

fn reaches_target(mut v_x: i32, mut v_y: i32, x_range: &std::ops::RangeInclusive<i32>, y_range: &std::ops::RangeInclusive<i32>) -> bool{
    let mut x = 0;
    let mut y = 0;

    loop{
        x += v_x;
        y += v_y;
        if v_x > 0{
            v_x -= 1;
        }
        v_y -= 1;

        if x_range.contains(&x) && y_range.contains(&y){
            return true;
        }
        if x > *x_range.end(){
            return false;
        }
        if y < *y_range.start(){
            return false;
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![String::from("target area: x=20..30, y=-10..-5")];

        let result = run(&input);

        assert_eq!(result, (45, 112));
    }
}