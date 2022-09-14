use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::time::Instant;

use crate::Answer;

pub fn day_14_main() -> Answer {
    let time_before = Instant::now();

    let lines = fs::read_to_string("src/days/day_14/input1.txt")
        .unwrap()
        .split('\n')
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();

    let mut lines_iter = lines.iter();

    let mut current_string = lines_iter.next().unwrap().trim().to_owned();

    //Skip empty line
    lines_iter.next();

    let mut mappings = HashMap::new();

    for line in lines_iter {
        let mut l = line.split(" -> ");
        mappings.insert(l.next().unwrap(), l.next().unwrap().chars().next().unwrap());
    }

    for _ in 0..10 {
        let mut new_line = String::new();

        for i in 0..current_string.len() - 1 {
            write!(
                new_line,
                "{}{}",
                &current_string[i..i + 1],
                mappings.get(&current_string[i..i + 2]).unwrap()
            )
            .unwrap();
        }
        new_line += &current_string[current_string.len() - 1..current_string.len()];
        println!("{}", new_line);
        current_string = new_line;
    }

    let mut frequencies = HashMap::new();

    for c in current_string.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    let mut frequencies = frequencies.iter().collect::<Vec<(&char, &i32)>>();

    frequencies.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let part_1 = frequencies.last().unwrap().1 - frequencies.first().unwrap().1;

    let part_2 = "B";

    let duration = Instant::now() - time_before;

    Answer {
        day: 14,
        part_1: part_1.to_string(),
        part_2: part_2.to_string(),
        duration,
    }
}
