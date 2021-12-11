use core::num;
use std::{fs};
use std::time::Instant;

use ndarray::{Array2, s};

use crate::Answer;

pub fn day_11_main() -> Answer{
    let time_before = Instant::now();

    let mut num_map:Array2<u32> = Array2::zeros((10,10));

    fs::read_to_string("src/days/day_11/input1.txt")
        .unwrap()
        .split("\n")
        .map(|line| line.trim().chars())
        .flatten()
        .enumerate()
        .for_each(|(pos,char)| num_map[[pos.div_euclid(10),pos%10]] = char.to_digit(10).unwrap());

    let mut flash_count = 0;


    // println!("{:?}",num_map.slice(s![0..2,0..-40.min(10).max(0)]));


    for _ in 0..100{
        num_map += 1;

        let mut set_to_flash:Vec<(usize,usize)> = vec![];
        let mut checked = 0;
        let mut checked_poses:Array2<u32> = Array2::zeros((10,10));

        for (pos,val) in num_map.iter().enumerate(){
            if val > &9 {
                set_to_flash.push((pos.div_euclid(10),pos%10))
            }
        }

        // println!("{:?}", num_map);

        while checked < set_to_flash.len(){
            let pos = set_to_flash[checked];

            if checked_poses[[pos.0,pos.1]] != 1{
                num_map.slice_mut(s![((pos.0 - 1) as i32).max(0)..(pos.0 + 2).min(10) as i32,((pos.1 - 1) as i32).max(0)..(pos.1 + 2).min(10) as i32]).iter_mut().for_each(|v| *v += 1);
                checked_poses[[pos.0,pos.1]] = 1;
            }
            
            checked += 1;

            if checked == set_to_flash.len(){
                for (pos_check,val) in num_map.iter().enumerate(){
                    if val > &9 && checked_poses[[pos_check.div_euclid(10),pos_check%10]] != 1{
                        set_to_flash.push((pos_check.div_euclid(10),pos_check%10))
                    }
                }
            }
        }

        num_map.iter_mut().for_each(|v| if *v > 9 {*v = 0});

        // println!("{:?}", num_map);


        // println!("{:?}",set_to_flash);

        flash_count += checked
    }


    let part_1 = flash_count;
    let part_2 = "B";

    let duration = Instant::now() - time_before;

    Answer{day:10, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}


fn get_slice(pos:(usize,usize)){
    
}