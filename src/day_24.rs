// use rayon::prelude::*;

pub fn run(_: &[String]) -> (u64, u64) {
    // (11111111111111_u64..=99999999999999_u64)
    // .into_par_iter()
    // .for_each(|x| {
    //     let s = x.to_string();
    //     if !s.contains('0') && simulate_code(&s) == 0 {
    //         println!("{}", x);
    //     }
    // });
    (91297395919993, 71131151917891) // SEE /README.md
}

fn _simulate_code_raw(input: &str) -> i64 {
    const Z_DIVISORS: [i64; 14] = [1, 1, 1, 1, 26, 1, 1, 26, 1, 26, 26, 26, 26, 26];
    const X_ADDERS: [i64; 14] = [14, 13, 15, 13, -2, 10, 13, -15, 11, -9, -9, -7, -4, -6];
    const Y_ADDERS: [i64; 14] = [0, 12, 14, 0, 3, 15, 11, 12, 1, 12, 3, 10, 14, 12];

    let mut chars = input.chars();
    let mut w;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for i in 0..14 {
        w = chars.next().unwrap().to_digit(10).unwrap() as i64; // inp w
        x *= 0; // mul x 0
        x += z; // add x z
        x %= 26; // mod x 26
        z /= Z_DIVISORS[i]; // div z Z_DIVISORS[i]
        x += X_ADDERS[i]; // add x X_ADDERS[i]
        x = if x == w { 1 } else { 0 }; // eql x w
        x = if x == 0 { 1 } else { 0 }; // eql x 0
        y *= 0; // mul y 0
        y += 25; // add y 25
        y *= x; // mul y x
        y += 1; // add y 1
        z *= y; // mul z y
        y *= 0; // mul y 0
        y += w; // add y w
        y += Y_ADDERS[i]; // add Y_ADDERS[i]
        y *= x; // mul y x
        z += y; // add z y
    }
    z
}

fn _simulate_code_readable(input: &str) -> i64 {
    const Z_DIVISORS: [bool; 14] = [
        false, false, false, false, true, false, false, true, false, true, true, true, true, true,
    ];
    const X_ADDERS: [i64; 14] = [14, 13, 15, 13, -2, 10, 13, -15, 11, -9, -9, -7, -4, -6];
    const Y_ADDERS: [i64; 14] = [0, 12, 14, 0, 3, 15, 11, 12, 1, 12, 3, 10, 14, 12];

    let mut chars = input.chars();
    let mut z = 0;
    for i in 0..14 {
        let w = chars.next().unwrap().to_digit(10).unwrap() as i64;
        if z % 26 + X_ADDERS[i] != w {
            if Z_DIVISORS[i] {
                z -= z % 26;
            } else {
                z *= 26;
            }
            z += w + Y_ADDERS[i];
        } else if Z_DIVISORS[i] {
            z /= 26;
        }
    }
    z
}

fn _simulate_code(input: &str) -> i64 {
    const Z_DIVISORS: [i64; 14] = [1, 1, 1, 1, 26, 1, 1, 26, 1, 26, 26, 26, 26, 26];
    const X_ADDERS: [i64; 14] = [14, 13, 15, 13, -2, 10, 13, -15, 11, -9, -9, -7, -4, -6];
    const Y_ADDERS: [i64; 14] = [0, 12, 14, 0, 3, 15, 11, 12, 1, 12, 3, 10, 14, 12];

    let mut chars = input.chars();
    let mut w;
    let mut x;
    let mut z = 0;
    for i in 0..14 {
        w = chars.next().unwrap().to_digit(10).unwrap() as i64;
        x = z % 26 + X_ADDERS[i];
        z /= Z_DIVISORS[i];
        if x != w {
            z *= 26;
            z += w + Y_ADDERS[i];
        }
    }
    z
}

fn _simulate_code_dbg(input: &str) -> i64 {
    const Z_DIVISORS: [i64; 14] = [1, 1, 1, 1, 26, 1, 1, 26, 1, 26, 26, 26, 26, 26];
    const X_ADDERS: [i64; 14] = [14, 13, 15, 13, -2, 10, 13, -15, 11, -9, -9, -7, -4, -6];
    const Y_ADDERS: [i64; 14] = [0, 12, 14, 0, 3, 15, 11, 12, 1, 12, 3, 10, 14, 12];

    println!("{}", input);
    let mut chars = input.chars();
    let mut z = 0;
    for i in 0..14 {
        let w = chars.next().unwrap().to_digit(10).unwrap() as i64;
        let x = z % 26 + X_ADDERS[i];
        z /= Z_DIVISORS[i];
        if x != w {
            z *= 26;
            z += w + Y_ADDERS[i];
        }
        println!("{}", _format_base_26(z as u64));
    }
    z
}

fn _format_base_26(mut num: u64) -> String {
    const CHAR_MAP: [char; 26] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    ];
    let mut result = String::new();
    if num == 0 {
        result.push('0');
    }
    while num > 0 {
        let digit = (num % 26) as usize;
        num /= 26;
        result.insert(0, CHAR_MAP[digit]);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = "11111111111111";
        let result = _simulate_code(input);
        println!("{}", result);
        let input = "91111111111111";
        let result = _simulate_code(input);
        println!("{}", result);
    }

    #[test]
    fn results_1() {
        let results = [
            91131151917893_u64,
            91131162917893_u64,
            91131173917893_u64,
            91131184917893_u64,
            91131195917893_u64,
            91131251918893_u64,
            91131262918893_u64,
            91131273918893_u64,
            91131284918893_u64,
            91131295918893_u64,
            91131351919893_u64,
            91131362919893_u64,
            91131373919893_u64,
            91131384919893_u64,
            91131395919893_u64,
        ];
        for i in results {
            let s = i.to_string();
            assert_eq!(_simulate_code(&s), 0);
        }
    }

    #[test]
    fn simulation_dbg() {
        let input = "91131395919893";
        _simulate_code_dbg(input);
    }

    #[test]
    fn my_answer() {
        let input = "91297395919993";
        assert_eq!(_simulate_code_dbg(input), 0);
    }

    #[test]
    fn my_answer_2() {
        let input = "71131151917891";
        assert_eq!(_simulate_code_dbg(input), 0);
    }
}
