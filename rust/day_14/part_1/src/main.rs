use std::fs;
use regex::Regex;

fn main() {
    let GRID_X = 101;
    let GRID_Y = 103;
    let SECONDS = 100;
    let mut quad_1 = 0;
    let mut quad_2 = 0;
    let mut quad_3 = 0;
    let mut quad_4 = 0;
    let re = Regex::new(r"-?\d+,-?\d+").unwrap();
    let input = fs::read_to_string("src/input.txt").unwrap();
    for line in input.lines() {
        let regex_result = re.find_iter(line).map(|x| x.as_str()).collect::<Vec<&str>>();
        let coords = regex_result[0].split(",").collect::<Vec<&str>>();
        let x = coords[0].parse::<i32>().unwrap();
        let y = coords[1].parse::<i32>().unwrap();
        let velocity = regex_result[1].split(",").collect::<Vec<&str>>();
        let vx = velocity[0].parse::<i32>().unwrap();
        let vy = velocity[1].parse::<i32>().unwrap();

        let new_x = ((x + (vx * SECONDS)) % GRID_X + GRID_X) % GRID_X;
        let new_y = ((y + (vy * SECONDS)) % GRID_Y + GRID_Y) % GRID_Y;

        let mid_x = GRID_X / 2;
        let mid_y = GRID_Y / 2;

        if new_x == mid_x || new_y == mid_y {
            continue;
        }

        if new_x < mid_x && new_y < mid_y {
            quad_1 += 1;
        } else if new_x > mid_x && new_y < mid_y {
            quad_2 += 1;
        } else if new_x < mid_x && new_y > mid_y {
            quad_3 += 1;
        } else if new_x > mid_x && new_y > mid_y {
            quad_4 += 1;
        }
    }
    let total = quad_1 * quad_2 * quad_3 * quad_4;
    println!("Total: {}", total);
}
