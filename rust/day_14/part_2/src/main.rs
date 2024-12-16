use std::fs;
use regex::Regex;

fn main() {
    let GRID_X = 101;
    let GRID_Y = 103;
    let SECONDS = 10000;
    let re = Regex::new(r"-?\d+,-?\d+").unwrap();
    let input = fs::read_to_string("src/input.txt").unwrap();

    for i in 7000..SECONDS {
        let mut robot_coords: Vec<(i32, i32)> = Vec::new();
        for line in input.lines() {
            let regex_result = re.find_iter(line).map(|x| x.as_str()).collect::<Vec<&str>>();
            let coords = regex_result[0].split(",").collect::<Vec<&str>>();
            let x = coords[0].parse::<i32>().unwrap();
            let y = coords[1].parse::<i32>().unwrap();
            let velocity = regex_result[1].split(",").collect::<Vec<&str>>();
            let vx = velocity[0].parse::<i32>().unwrap();
            let vy = velocity[1].parse::<i32>().unwrap();

            let new_x = ((x + (vx * i)) % GRID_X + GRID_X) % GRID_X;
            let new_y = ((y + (vy * i)) % GRID_Y + GRID_Y) % GRID_Y;

            robot_coords.push((new_x, new_y));
        }
        let mut grid = vec![vec!['.'; GRID_X as usize]; GRID_Y as usize];
        for coord in robot_coords {
            grid[coord.1 as usize][coord.0 as usize] = '*';
        }

        for row in grid {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
        println!("Second: {}", i);
        println!("Press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!();
}
}
