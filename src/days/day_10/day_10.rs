use std::{fs, vec};
use std::time::Instant;

use crate::Answer;

pub fn day_10_main() -> Answer{
    let time_before = Instant::now();

    // Turn lines into a vec of strings
    let lines = fs::read_to_string("src/days/day_10/input1.txt")
                                 .unwrap()
                                 .split("\n")
                                 .map(|line| line.trim().to_owned())
                                 .collect::<Vec<String>>();


    println!("{:?}", is_valid_line(&"[{[{({}]{}}([{[{{{}}([]".to_owned()));


    let part_1 = lines.iter()
         .map(|line| is_valid_line(line))
         .filter(|opt| opt.is_some())
         .map(|opt| opt.unwrap())
         .inspect(|c| println!("{}",c))
         .map(|c| match c {')' => 3,']' => 57,'}' => 1197,'>' => 25137, _ =>  0})
         .sum::<u32>();

    let part_2 = "B";

    let duration = Instant::now() - time_before;

    Answer{day:10, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}


fn is_valid_line(line:&String) -> Option<char>{
    let mut last_open_vec:Vec<char> = vec![];
    let mut valid = true;
    let mut invalid_char = ' ';

    for c in line.chars(){
        if valid{
            match c {
                '{' | '[' | '(' | '<' => last_open_vec.push(c),
                '}' => {if !(last_open_vec.len() > 0 && last_open_vec.pop().unwrap() == '{') {valid = false; invalid_char = c}},
                ']' => {if !(last_open_vec.len() > 0 && last_open_vec.pop().unwrap() == '[') {valid = false; invalid_char = c}},
                ')' => {if !(last_open_vec.len() > 0 && last_open_vec.pop().unwrap() == '(') {valid = false; invalid_char = c}},
                '>' => {if !(last_open_vec.len() > 0 && last_open_vec.pop().unwrap() == '<') {valid = false; invalid_char = c}},
                _ => valid = false
            }
        }
    }


    if valid{
        return None
    }
    else{
        return Some(invalid_char)
    }
}