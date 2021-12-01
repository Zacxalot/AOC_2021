use std::fs;

pub fn day_1_main(){
    // Part 1
    // Get contents, do lazy unwrap because we know the file is there and the values are all i32.
    let contents = fs::read_to_string("src/days/day_1/input1.txt")
                               .unwrap()
                               .split("\n")
                               .map(|x| x.trim().parse::<u32>().unwrap())
                                // .map(|x| String::from(x))   
                            .collect::<Vec<u32>>();
    
    // Get a sliding window and if the first val is smaller, map to 1,
    // then get the sum.
    let larger = contents.windows(2)
                             .map(|vals| if vals[0] < vals[1] {1} else {0})
                             .sum::<i32>();

    println!("{}",larger);
}