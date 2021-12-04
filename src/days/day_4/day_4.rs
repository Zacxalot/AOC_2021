use std::fs;
use std::time::Instant;


use ndarray::{Array2, Axis};

use crate::Answer;
#[derive(Clone)]
struct BingoCard{
    card:Array2<(bool,u32)>,
    colcount:[u32;5],
    rowcount:[u32;5],
}

impl BingoCard{

    fn update_card(&mut self, num:u32) -> bool{

        for ((row, col), (marked, val)) in self.card.indexed_iter_mut(){
            if num == *val{
                self.colcount[col] += 1;
                self.rowcount[row] += 1;
                *marked = true;
                
            }
        }
        
        if self.colcount.contains(&5) || self.rowcount.contains(&5){
            return true
        }

        false
    }
}

pub fn day_4_main() -> Answer{
    let time_before = Instant::now();

    let contents = fs::read_to_string("src/days/day_4/input1.txt").unwrap();

    let numberlist = contents.split("\n").next().unwrap().split(',').map(|x| x.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let cards_strings = contents.split("\n\r").map(|card| card.to_string()).skip(1).collect::<Vec<String>>();
    let mut card_vec:Vec<BingoCard> = vec![];


    // Convert all of the card strings into a vec of bingo card structs
    for card in cards_strings{
        let mut arr = Array2::<(bool,u32)>::default((5,5));

        let lines = card.trim().split("\n").map(|line| line.trim().split_whitespace().map(|val| val.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>());
        for (mut row, line) in arr.axis_iter_mut(Axis(0)).zip(lines){

            for (i, col) in row.iter_mut().enumerate(){
                *col = (false,*line.get(i).unwrap());
            }
        }

        card_vec.push(BingoCard{card:arr, colcount: [0;5], rowcount: [0;5]});
    }


    // Copy cards for part 2
    let mut card_vec_2 = card_vec.to_vec();

    let mut won = false;
    let mut part_1 = 0;


    // Loop through the bingo numbers until we get a winner
    for val in &numberlist{
        for card in card_vec.iter_mut(){
            if !won{
                won = card.update_card(*val);

                if won {
                    let remaining = card.card.iter().filter(|val| !val.0).map(|val| val.1).sum::<u32>();
                    part_1 = remaining * val;
                }
            }   
        }
    }


    let mut part_2 = 0;

    let mut had:Vec<usize> = vec![];

    // Loop through all of the bingo numbers
    
    for val in &numberlist{
        println!("{}",val);
        for (i,card) in card_vec_2.iter_mut().enumerate(){
            won = card.update_card(*val);
            
            // Checks if the card has won before
            if won && !had.contains(&i){
                // Finds the remaining numbers by filtering ones that don't have markers on them
                let remaining = card.card.iter().filter(|val| !val.0).map(|val| val.1).sum::<u32>();
                part_2 = remaining * val;
                had.push(i);
            }
        }
    }

    let duration = Instant::now() - time_before;

    Answer{day:4, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}