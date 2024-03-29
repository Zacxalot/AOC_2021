use std::collections::HashMap;
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

    let current_string = lines_iter.next().unwrap().trim().to_owned();

    let pairs = current_string
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|chars| chars.iter().cloned().collect::<String>())
        .collect::<Vec<String>>();

    let first_and_last = [
        current_string.chars().next().unwrap(),
        current_string.chars().next_back().unwrap(),
    ];

    //Skip empty line
    lines_iter.next();

    let mut mappings: HashMap<String, (String, String)> = HashMap::new();
    let mut counts: HashMap<String, usize> = HashMap::new();

    for line in lines_iter {
        let mut split = line.split(" -> ");

        let key = split.next().unwrap();
        let mut front = key.chars();
        let back = split.next().unwrap().chars().next().unwrap();

        let left = [front.next().unwrap(), back]
            .iter()
            .cloned()
            .collect::<String>();

        let right = [back, front.next().unwrap()]
            .iter()
            .cloned()
            .collect::<String>();
        mappings.insert(key.to_owned(), (left, right));
    }

    for pair in pairs {
        *counts.entry(pair).or_default() += 1;
    }

    let mut part_1 = 0;

    for i in 0..40 {
        counts = iterate(&counts, &mappings);
        if i == 9 {
            part_1 = counts_to_answer(&counts, &first_and_last);
        }
    }

    let part_2 = counts_to_answer(&counts, &first_and_last);

    let duration = Instant::now() - time_before;

    Answer {
        day: 14,
        part_1: part_1.to_string(),
        part_2: part_2.to_string(),
        duration,
    }
}

fn iterate(
    counts: &HashMap<String, usize>,
    mappings: &HashMap<String, (String, String)>,
) -> HashMap<String, usize> {
    let mut changes: HashMap<String, usize> = HashMap::new();
    let mut total_count = 0;
    for (key, count) in counts {
        total_count += count;
        let (map_1, map_2) = mappings.get(key).unwrap();

        *changes.entry(map_1.clone()).or_default() += count;
        *changes.entry(map_2.clone()).or_default() += count;
    }

    changes
}

fn counts_to_answer(counts: &HashMap<String, usize>, first_and_last: &[char; 2]) -> usize {
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for (key, count) in counts {
        let mut chars = key.chars();
        let char_1 = chars.next().unwrap();
        let char_2 = chars.next().unwrap();
        *char_counts.entry(char_1).or_default() += count;
        *char_counts.entry(char_2).or_default() += count;
    }

    for (c, count) in char_counts.iter_mut() {
        if first_and_last.contains(c) {
            *count += 1;
        }

        *count /= 2;
    }

    let mut sorted = char_counts.values().collect::<Vec<&usize>>();
    sorted.sort();
    *sorted.last().unwrap() - *sorted.first().unwrap()
}
