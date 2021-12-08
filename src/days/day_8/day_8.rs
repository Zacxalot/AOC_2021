use std::fs;
use std::time::Instant;

use crate::Answer;

pub fn day_8_main() -> Answer{
    let time_before = Instant::now();

    // Get our 2d vec of strings
    let contents = fs::read_to_string("src/days/day_8/input1.txt")
                      .unwrap()
                      .split("\n")
                      .map(|line| line.split(&[' ','|','\r'][..])
                                           .filter(|val| val != &"")
                                           .map(|val| val.to_string())
                                           .collect::<Vec<String>>())
                      .collect::<Vec<Vec<String>>>();

    let part_1 = contents.iter()
                         .map(|line| line[10..14].iter().filter(|digit| [2,3,4,7].contains(&digit.len())).count())
                         .sum::<usize>();

    // for line in contents{
    //     println!("{:?}",line);
    // }

    let part_2 = "B";

    let duration = Instant::now() - time_before;

    Answer{day:8, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}