use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::error::Error;

pub fn read_lines<P>(filename: P) -> Result<Vec<String>, Box<dyn Error>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let line_iterator = io::BufReader::new(file).lines();

    Ok(line_iterator.collect::<Result<_, _>>()?)
}

// Takes a string with {} and a string without and matches parts of the second string to the {} in the first string
// NOTE: Does not work well if {}{} are right next to one another
pub fn deformat_str(format_str: &str, actual_str: &str) -> Option<Vec<String>>{
    let actual_str = format!("{}{}", actual_str, "|END|");
    let format_str = format!("{}{}", format_str, "|END|");
    
    let mut in_brackets = false;    
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
            continue;
        }
        if !in_brackets{
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

    for i in 1..in_betweens.len(){
        let val: String = String::from(operable_str.split(&in_betweens[i]).collect::<Vec<&str>>()[0]);
        operable_str = operable_str[val.len()+in_betweens[i].len()..].to_string();
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
