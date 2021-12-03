use std::{fs, time::Instant};

use crate::Answer;

pub fn day_3_main() -> Answer{
    let time_before = Instant::now();

    let contents = fs::read_to_string("src/days/day_3/input1.txt")
                    .unwrap()
                    .split("\n")
                    .map(|x| x.trim().to_string().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>())
                    .collect::<Vec<Vec<u32>>>();

    let bits_len = contents.first().unwrap().len();
    let majority_target = (contents.len()/2) as u32;
    let mut common:Vec<usize> = vec![];
    let mut uncommon:Vec<usize> = vec![];

    for i in 0..bits_len{
        let sum = contents.iter().map(|x| x.get(i).unwrap()).sum::<u32>();
        if sum > majority_target{
            common.push(2_usize.pow((bits_len-i-1) as u32));
            uncommon.push(0);
        }
        else{
            common.push(0);
            uncommon.push(2_usize.pow((bits_len-i-1) as u32));
        }
    }

    let part_1 = common.iter().sum::<usize>() * uncommon.iter().sum::<usize>();

    
    let duration = Instant::now() - time_before;

    Answer{day:3, part_1:part_1.to_string(), part_2:"b".to_string(), duration:duration}
}