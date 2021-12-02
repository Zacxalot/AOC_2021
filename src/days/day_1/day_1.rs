use std::fs;
use std::time::Instant;

pub fn day_1_main() -> String{
    let time_before = Instant::now();

    // Get contents, do lazy unwrap because we know the file is there and the values are all u32.
    let contents = fs::read_to_string("src/days/day_1/input1.txt")
                      .unwrap()
                      .split("\n")
                      .map(|x| x.trim().parse::<u32>().unwrap())
                      .collect::<Vec<u32>>();

    // Part one
    let larger_part_1 = get_larger_count(&contents);

    // Part two is the same as part 1 after we get the sum in each of the 3 width sliding windows
    let large_windows_sums = contents.windows(3)
                                     .map(|vals| vals[0] + vals[1] + vals[2])
                                     .collect::<Vec<u32>>();
    let larger_part_2 = get_larger_count(&large_windows_sums);

    let duration = Instant::now() - time_before;

    format!("Day 1: Part 1 - {} Part 2 - {} Duration - {}μs", larger_part_1, larger_part_2, duration.as_micros())
}

// Get a sliding window and if the first val is smaller, map to 1, then get the sum.
fn get_larger_count(values:&Vec<u32>) -> u32{
    values.windows(2)
          .map(|vals| if vals[0] < vals[1] {1} else {0})
          .sum::<u32>()
}