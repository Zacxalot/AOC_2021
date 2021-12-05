use std::fs;
use std::time::Instant;



use crate::Answer;

#[derive(Debug)]
struct Line{
    x1:i32,
    y1:i32,
    x2:i32,
    y2:i32,
}

pub fn day_5_main() -> Answer{
    let time_before = Instant::now();
    let mut lines:Vec<Line> = vec![];
    
    let mut max_x = 0;
    let mut max_y = 0;

    for line in fs::read_to_string("src/days/day_5/input1.txt").unwrap().split("\n"){
        
        let vals = line.split(|c| c == ' ' || c == '-' || c == '>' || c == ',').map(|val|  val.trim()).filter(|c| c.len() > 0).map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        // let vals = line.split(|c| c == ' ' || c == '-' || c == '>' || c == ',').map(|val|  val.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let new_line = Line{
            x1:*vals.get(0).unwrap(),
            y1:*vals.get(1).unwrap(),
            x2:*vals.get(2).unwrap(),
            y2:*vals.get(3).unwrap()
        }; 

        if new_line.x1 > max_x {max_x = new_line.x1};
        if new_line.x2 > max_x {max_x = new_line.x2};
        if new_line.y1 > max_y {max_y = new_line.y1};
        if new_line.y2 > max_y {max_y = new_line.y2};

        lines.push(new_line);
    }

    let mut vent_map_1 = (0..max_y+1).map(|_| vec![0;1 + max_x as usize]).collect::<Vec<Vec<i32>>>();

    for line in lines.iter().filter(|line| line.x1 == line.x2 || line.y1 == line.y2){
        for point in calculate_line(&line){
            *vent_map_1.get_mut(point.1).unwrap().get_mut(point.0).unwrap() += 1;
        }
    }

    let part_1 = vent_map_1.iter().flatten().filter(|val| val >= &&2).count();

    
    let part_2 = "B";


    let duration = Instant::now() - time_before;

    Answer{day:4, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}

fn calculate_line(line:&Line) -> Vec<(usize,usize)>{
    let mut out_line:Vec<(usize,usize)> = vec![];

    if line.x1 - line.x2 != 0{
        let mut x = line.x1;
        let x_step = if line.x1 > line.x2 {-1} else {1};
        let mut counter = 0;

        while x != line.x2 + x_step{

            out_line.push((x as usize,(line.y1 + (((line.y2 - line.y1)*counter)/(line.x2-line.x1))) as usize));

            x += x_step;
            counter += 1;
        }
    }

    if line.y1 - line.y2 != 0{
        let mut y = line.y1;
        let y_step = if line.y1 > line.y2 {-1} else {1};
        let mut counter = 0;

        while y != line.y2 + y_step{

            out_line.push(((line.x1 + (((line.x2 - line.x1)*counter)/(line.y2-line.y1))) as usize,y as usize));

            y += y_step;
            counter += 1;
        }
    }
    
    out_line.sort();
    out_line.dedup_by(|a,b| a.0 == b.0 && a.1 == b.1);
    out_line
}

