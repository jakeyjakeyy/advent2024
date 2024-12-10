use std::fs;
fn main() {
    let file = fs::read_to_string("src/input.txt").unwrap();
    let lines = file.lines();
    let mut arr_2d: Vec<Vec<String>> = Vec::new();
    for line in lines {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        arr_2d.push(row);
    }

    let mut trails = 0;
    for i in 0..arr_2d.len() {
        for j in 0..arr_2d[i].len() {
            if arr_2d[i][j] == "0" {
                let mut found_trails: Vec<(usize, usize)> = Vec::new();
                let (res, _pos) = find_trails(&arr_2d, i, j, &mut found_trails);
                trails += res;
            }
        }
    }
    println!("Total trails: {}", trails);

}

fn find_trails(arr_2d: &Vec<Vec<String>>, row: usize, col: usize, found_trail_endings: &mut Vec<(usize, usize)>) -> (i32, Vec<(usize, usize)>) {
    let directions = ["up", "down", "left", "right"];

    let mut found_trails = 0;
    let mut trail_endings = found_trail_endings.clone();
    let current_elevation = &arr_2d[row][col].parse::<i32>().unwrap();
    
    if *current_elevation == 9 {
        // if trail_endings.contains(&(row, col)) {
        //     return (0, trail_endings);
        // }
        trail_endings.push((row, col));
        return (1, trail_endings);
    }

    for direction in directions.iter() {
        let res: i32;
        let endings_res: Vec<(usize, usize)>;
        match *direction {
            "up" => {
                if row <= 0 || arr_2d[row - 1][col] != (current_elevation + 1).to_string() {
                    continue;
                }
                (res, endings_res) = find_trails(arr_2d, row - 1, col, &mut trail_endings);
            }
            "down" => {
                if row >= arr_2d.len() - 1 || arr_2d[row + 1][col] != (current_elevation + 1).to_string() {
                    continue;
                }
                (res, endings_res) = find_trails(arr_2d, row + 1, col, &mut trail_endings);
            }
            "left" => {
                if col <= 0 || arr_2d[row][col - 1] != (current_elevation + 1).to_string() {
                    continue;
                }
                (res, endings_res) = find_trails(arr_2d, row, col - 1, &mut trail_endings);
            }
            "right" => {
                if col >= arr_2d[row].len() - 1 || arr_2d[row][col + 1] != (current_elevation + 1).to_string() {
                    continue;
                }
                (res, endings_res) = find_trails(arr_2d, row, col + 1, &mut trail_endings);
            }
            _ => continue,
        }
        found_trails += res;
        trail_endings = endings_res;
    }
    (found_trails, trail_endings)
}