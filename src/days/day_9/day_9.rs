use std::fs;
use std::time::Instant;

use ndarray::Array2;

use crate::Answer;

pub fn day_9_main() -> Answer{
    let time_before = Instant::now();
    let contents = fs::read_to_string("src/days/day_9/input1.txt").unwrap().split("\n").map(|line| line.trim().to_owned()).collect::<Vec<String>>();

    let width = contents[0].len();
    let height = contents.len();
    let mut map:Array2<u8> = Array2::zeros((width,height));

    for (y,line) in contents.iter().enumerate(){
        for (x,val) in line.chars().enumerate(){
            map[[x,y]] = val.to_digit(10).unwrap() as u8;
        }
    }

    let mut lowpoints:Vec<u32> = vec![];
    for y in 0..height{
        for x in 0..width{
            let current_val = map[[x,y]];
            let mut low = true;
            if x > 0 && map[[x-1,y]] <= current_val{
                low = false;
            }

            if low && x < width - 1 && map[[x+1,y]] <= current_val{
                low = false;
            }

            if low && y > 0 && map[[x,y-1]] <= current_val{
                low = false;
            }

            if low && y < height - 1 && map[[x,y+1]] <= current_val{
                low = false;
            }

            if low{
                lowpoints.push(current_val as u32);
            }
        }
    }

    let part_1:u32 = lowpoints.iter().map(|val| val + 1).sum();
    let part_2 = "B";

    let duration = Instant::now() - time_before;

    Answer{day:9, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}