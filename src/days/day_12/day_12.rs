use std::collections::HashMap;
use std::fs;
use std::time::Instant;


use crate::Answer;

pub fn day_12_main() -> Answer{
    let time_before = Instant::now();

    // Get all of the pairs from the list
    let pairs = fs::read_to_string("src/days/day_12/input1.txt")
        .unwrap()
        .split("\n")
        .map(|line| line.trim().split("-").map(|s| s.to_owned()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

    let mut point_paths:HashMap<&str,Vec<&str>> = HashMap::new();

    for val in pairs.iter(){
        let key1 = point_paths.entry(&val[0]).or_insert(vec![]);
        if val[1] != "start"{
            key1.push(&val[1]);
        }
        
        
        let key2 = point_paths.entry(&val[1]).or_insert(vec![]);
        if val[0] != "start"{
            key2.push(&val[0]);
        }
    }


    let mut paths_to_check = point_paths.get("start").unwrap().iter().map(|v| vec!["start",v]).collect::<Vec<Vec<&str>>>();

    let mut paths = 0;

    while paths_to_check.len() > 0{
        let next_path = paths_to_check.pop().unwrap();
        
        for potential_destinations in point_paths.get(next_path.last().unwrap()){
            for destination in potential_destinations{
                if *destination == "end"{
                    paths += 1;
                }
                else if destination.chars().next().unwrap().is_uppercase() || !next_path.contains(destination){
                    let mut path = next_path.clone();
                    path.push(destination);
                    paths_to_check.push(path);
                }
            }
        }
    }


    let mut paths_to_check = point_paths.get("start").unwrap().iter().map(|v|( vec!["start",v],false)).collect::<Vec<(Vec<&str>,bool)>>();

    let mut paths2 = 0;

    while paths_to_check.len() > 0{
        let next_path = paths_to_check.pop().unwrap();
        
        for potential_destinations in point_paths.get(next_path.0.last().unwrap()){
            for destination in potential_destinations{
                if *destination == "end"{
                    // println!("{:?}",next_path);
                    paths2 += 1;
                }
                else if destination.chars().next().unwrap().is_uppercase() || !next_path.0.contains(destination){
                    let mut path = next_path.clone();
                    path.0.push(destination);
                    paths_to_check.push(path);
                }
                else if next_path.1 == false{
                    let mut path = next_path.clone();
                    path.0.push(destination);
                    path.1 = true;
                    paths_to_check.push(path);
                }
            }
        }
    }

    let part_1 = paths;
    let part_2 = paths2;

    let duration = Instant::now() - time_before;

    Answer{day:12, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}