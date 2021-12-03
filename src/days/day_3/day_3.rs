use std::{fs, time::Instant};

use crate::Answer;

pub fn day_3_main() -> Answer{
    let time_before = Instant::now();

    // Get the lines out
    let lines = fs::read_to_string("src/days/day_3/input1.txt")
                    .unwrap()
                    .split("\n")
                    .map(|line| line.trim().to_string())
                    .collect::<Vec<String>>();

    let line_len = lines[0].len();

    // Turn the all of the lines into a big vec of u32
    let bits = lines.iter().map(|line| line.chars().map(|chr| chr.to_digit(10).unwrap())).flatten().collect::<Vec<u32>>();


    // Transpose the array into columns
    let mut columns:Vec<Vec<u32>> = vec![];
    for i in 0..line_len{
        columns.push(bits.iter()
                         .skip(i)
                         .step_by(line_len)
                         .map(|x| *x)
                         .collect::<Vec<u32>>());
    }

    // Part 1
    // Find find the highest frequency in each column
    let mut common:Vec<u32> = vec![];
    for val in columns.iter().map(|column| column.iter().sum::<u32>()){
        if val > (lines.len() / 2) as u32{
            common.push(1);
        }
        else{
            common.push(0);
        }
    }

    // Calculate answer
    let part_1 = vec_to_num(&common) * vec_to_num(&invert_vec(&common));
    
    // Part 2
    
    let duration = Instant::now() - time_before;
    
    Answer{day:3, part_1:part_1.to_string(), part_2:"b".to_string(), duration:duration}
}

fn vec_to_num(vals:&Vec<u32>) -> u32{
    let mut sum = 0;

    for (i,val) in vals.iter().enumerate(){
        sum += val * 2_u32.pow((vals.len()-i-1) as u32);
    }

    sum
}

fn invert_vec(vals:&Vec<u32>) -> Vec<u32>{
    let mut return_vec:Vec<u32> = vec![];

    for val in vals{
        if *val >= 1{
            return_vec.push(0)
        }
        else {
            return_vec.push(1)
        }
    }

    return_vec
}