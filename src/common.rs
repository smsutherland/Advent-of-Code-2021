use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Result<Vec<String>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let line_iterator = io::BufReader::new(file).lines();

    Ok(line_iterator.collect::<Result<_, _>>()?)
}

pub fn download_input(day_num: u8) -> Result<(), Box<dyn Error>> {
    let aoc_session = dotenv::var("AOC_SESSION").unwrap();

    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        "cookie",
        reqwest::header::HeaderValue::from_str(format!("session={}", aoc_session).as_str())?,
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(header)
        .build()?;
    let input = client
        .get(format!(
            "https://adventofcode.com/2021/day/{}/input",
            day_num
        ))
        .send()?
        .text()?;

    if input.starts_with("Please don't repeatedly request this endpoint before it unlocks!") {
        println!("Input for day {} not ready.", day_num);
        return Ok(());
    }

    let path_str = format!("data/input-{}.txt", day_num);
    let path = Path::new(&path_str);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(path)?;
    write!(file, "{}", input)?;

    println!("Downloaded input for day {}.", day_num);
    Ok(())
}

// Takes a string with {} and a string without and matches parts of the second string to the {} in the first string
// NOTE: Does not work well if {}{} are right next to one another
pub fn deformat_str(format_str: &str, actual_str: &str) -> Option<Vec<String>> {
    let actual_str = format!("{}{}", actual_str, "|END|");
    let format_str = format!("{}{}", format_str, "|END|");

    let mut in_brackets = false;
    let mut in_betweens: Vec<String> = Vec::new();
    let mut current_in_between: String = String::new();

    for c in format_str.chars() {
        if c == '{' {
            if in_brackets {
                return None;
            }
            in_brackets = true;
            in_betweens.push(current_in_between);
            current_in_between = String::new();
            continue;
        }
        if c == '}' {
            if !in_brackets {
                return None;
            }
            in_brackets = false;
            continue;
        }
        if !in_brackets {
            current_in_between.push(c);
        }
    }
    in_betweens.push(current_in_between);
    if in_brackets {
        return None;
    }

    if !actual_str.starts_with(in_betweens[0].as_str()) {
        return None;
    }
    if !actual_str.ends_with(in_betweens[in_betweens.len() - 1].as_str()) {
        return None;
    }

    let mut operable_str: String = actual_str[in_betweens[0].len()..].to_string();

    let mut result: Vec<String> = Vec::new();

    for i in 1..in_betweens.len() {
        let val: String =
            String::from(operable_str.split(&in_betweens[i]).collect::<Vec<&str>>()[0]);
        operable_str = operable_str[val.len() + in_betweens[i].len()..].to_string();
        result.push(val);
    }

    return Some(result);
}

/*
fn deformat_str(format_str: &str, actual_str: &str) -> Option<(Vec<String>, Vec<String>)>{
    let actual_str = format!("{}{}", actual_str, "|END|");
    let format_str = format!("{}{}", format_str, "|END|");

    let mut in_brackets = false;

    let mut type_strs: Vec<String> = Vec::new();
    let mut current_type = String::new();

    let mut in_betweens: Vec<String> = Vec::new();
    let mut current_in_between: String = String::new();

    for c in format_str.chars(){
        if c == '{'{
            if in_brackets{
                return None;
            }
            in_brackets = true;
            in_betweens.push(current_in_between);
            current_in_between = String::new();
            continue;
        }
        if c == '}'{
            if !in_brackets{
                return None;
            }
            in_brackets = false;
            type_strs.push(current_type);
            current_type = String::new();
            continue;
        }
        if in_brackets{
            current_type.push(c);
        }
        else{
            current_in_between.push(c);
        }
    }
    in_betweens.push(current_in_between);
    if in_brackets{
        return None;
    }

    if !actual_str.starts_with(in_betweens[0].as_str()){
        return None;
    }
    if !actual_str.ends_with(in_betweens[in_betweens.len()-1].as_str()){
        return None;
    }

    let mut operable_str: String = actual_str[in_betweens[0].len()..].to_string();

    let mut result: Vec<String> = Vec::new();

    for i in 1..=type_strs.len(){
        let val: String = String::from(operable_str.split(&in_betweens[i]).collect::<Vec<&str>>()[0]);
        operable_str = operable_str[val.len()+in_betweens[i].len()..].to_string();
        result.push(val);
    }

    return Some((result, type_strs));
}
*/

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn deformat_str_1() {
        let format_str = String::from("{},{} -> {},{}");
        let actual_str = String::from("111,863 -> 111,298");
        let result = deformat_str(&format_str, &actual_str).unwrap();

        let expected_result = vec![
            String::from("111"),
            String::from("863"),
            String::from("111"),
            String::from("298"),
        ];

        assert_eq!(result, expected_result);
    }
}
