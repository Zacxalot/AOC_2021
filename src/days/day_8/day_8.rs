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

    for line in contents{
        println!("{:?}",line);
    }

    let part_2 = "B";

    let duration = Instant::now() - time_before;

    Answer{day:8, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}

fn str_to_digit(input:&str) -> char{
    let len = input.len();
    let contains_c = input.contains('c');
    let contains_e = input.contains('e');

    match (len,contains_c,contains_e){
        (6,true,true)   => '0',
        (2,_,_)         => '1',
        (5,true,true)   => '2',
        (5,true,false)  => '3',
        (4,_,_)         => '4',
        (5,false,false) => '5',
        (6,false,true)  => '6',
        (3,_,_)         => '7',
        (7,_,_)         => '8',
        (6,true,false)  => '9',
        _ => 'X'
    }
}

#[test]
fn str_to_digit_test(){
    assert_eq!('0',str_to_digit("abcefg"));
    assert_eq!('1',str_to_digit("cf"));
    assert_eq!('2',str_to_digit("acdeg"));
    assert_eq!('3',str_to_digit("acdfg"));
    assert_eq!('4',str_to_digit("bcdf"));
    assert_eq!('5',str_to_digit("abdfg"));
    assert_eq!('6',str_to_digit("abdefg"));
    assert_eq!('7',str_to_digit("acf"));
    assert_eq!('8',str_to_digit("abcdefg"));
    assert_eq!('9',str_to_digit("abcdfg"));
}


/*
ab      = 1
abd     = 7
abef    = 4
bcdef   = 5
acdfg   = 2
abcdf   = 3
abcdef  = 9
bcdefg  = 6
abcdeg  = 0
abcdefg = 8


// Implement this!
find 3 (5 chars, superset of 1) - abcdf
find 9 (6 chars, superset of 3) - abcdef
find 5 (5 chars, subset of 9) - bcdef
find 2 (5 chars, not equal to 3 or 9) - acdfg
find what maps to c (2 AND 1) -> a
find 6 (6 characters, doesn't have a mapping to c (a in this case)) - abcdeg
find 0 (6 characters, not equal to 9 or 6)

d => a      7 take 1
ef => bd    4 take 1


5 3 5 3
bcdef abcdf bcdef abcdf
cdfeb fcadb cdfeb cdbaf
*/