use std::{fs, vec};
use std::time::Instant;

use crate::Answer;

#[derive(Debug)]
enum SyntaxLineResult{
    Invalid(char),
    Incomplete(u64)
}

pub fn day_10_main() -> Answer{
    let time_before = Instant::now();

    // Turn lines into a vec of strings
    let lines = fs::read_to_string("src/days/day_10/input1.txt")
                                 .unwrap()
                                 .split("\n")
                                 .map(|line| line.trim().to_owned())
                                 .collect::<Vec<String>>();

    let mut invalid_chars:Vec<char> = vec![];
    let mut incomplete_vals:Vec<u64> = vec![];

    for line in lines.iter().map(|line| is_valid_line(line)){
        match line {
            SyntaxLineResult::Invalid(c) => invalid_chars.push(c),
            SyntaxLineResult::Incomplete(v) => incomplete_vals.push(v),
        }
    }

    let part_1 = invalid_chars.iter()
                                  .map(|c| match c {')' => 3,']' => 57,'}' => 1197,'>' => 25137, _ =>  0})
                                  .sum::<u64>();

    incomplete_vals.sort();
    let part_2 = incomplete_vals.get(incomplete_vals.len()/2).unwrap();


    let duration = Instant::now() - time_before;

    Answer{day:10, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}


fn is_valid_line(line:&String) -> SyntaxLineResult{
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
        last_open_vec.reverse();
        let incomplete = last_open_vec.iter()
                                          .map(|c| match c {'(' => 1, '[' => 2, '{' => 3, '<' => 4, _ => 0})
                                          .fold(0,|acc,val| acc * 5 + val);
        return SyntaxLineResult::Incomplete(incomplete)
    }
    else{
        return SyntaxLineResult::Invalid(invalid_char)
    }
}