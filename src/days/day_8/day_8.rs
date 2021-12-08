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
                                           .map(|val| digit_chars_to_num(val))
                                           .map(|val| (val,val.count_ones() as u8))
                                           .collect::<Vec<(u8,u8)>>())
                      .collect::<Vec<Vec<(u8,u8)>>>();

    let part_1 = contents.iter()
                         .map(|line| line[10..14].iter().filter(|(_,ones)| [2,3,4,7].contains(ones)).count())
                         .sum::<usize>();

    let part_2 = contents.iter().map(|digits| deduce_values(digits)).sum::<u32>();

    let duration = Instant::now() - time_before;

    Answer{day:8, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}

fn deduce_values(digits:&Vec<(u8,u8)>) -> u32{
    let mut values:[u8; 10] = [0;10];
    
    values[1] = digits.iter().find(|(_,ones)| ones == &2).unwrap().0;
    values[4] = digits.iter().find(|(_,ones)| ones == &4).unwrap().0;
    values[7] = digits.iter().find(|(_,ones)| ones == &3).unwrap().0;
    values[8] = digits.iter().find(|(_,ones)| ones == &7).unwrap().0;
    values[3] = digits.iter().find(|(val,ones)| ones == &5 && (val ^ values[1]).count_ones() == 3).unwrap().0;
    values[9] = digits.iter().find(|(val,ones)| ones == &6 && (val ^ values[3]).count_ones() == 1).unwrap().0;
    values[5] = digits.iter().find(|(val,ones)| ones == &5 && (val ^ values[9]).count_ones() == 1 && val != &values[3]).unwrap().0;
    values[2] = digits.iter().find(|(val,ones)| ones == &5 && val != &values[3] && val != &values[5]).unwrap().0;
    let c = values[2] & values[1];
    values[6] = digits.iter().find(|(val,ones)| ones == &6 && val & c == 0).unwrap().0;
    values[0] = digits.iter().find(|(val,ones)| ones == &6 && val != &values[9] && val != &values[6]).unwrap().0;

    let mut digit_string = "".to_string();

    for (digit,_) in digits[10..14].iter(){
        digit_string += &values.iter().enumerate().find(|(_, val)| val == &digit).unwrap().0.to_string();
    }

    digit_string.parse::<u32>().unwrap()
}

fn digit_chars_to_num(line:&str) -> u8{
    let mut out:u8 = 0;

    for char in line.chars(){
        out += match char{
            'a' => 0b1000000,
            'b' => 0b0100000,
            'c' => 0b0010000,
            'd' => 0b0001000,
            'e' => 0b0000100,
            'f' => 0b0000010,
            'g' => 0b0000001,
            _ => 0b1111111
        }
    }

    out
}