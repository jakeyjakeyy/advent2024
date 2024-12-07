use std::fs;

fn main() {
    // Read input and make 2d array
    let input: String = fs::read_to_string("src/input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.lines().collect();
    let mut arr_2d: Vec<Vec<char>> = Vec::new();
    for line in lines {
        arr_2d.push(line.chars().collect());
    }

    // Find the starting position and direction
    let mut dir = "up";

    let mut pos: Vec<usize> = vec![0, 0];
    for i in 0..arr_2d.len() {
        for j in 0..arr_2d[i].len() {
            if arr_2d[i][j] == '^' {
                dir = "up";
                pos = vec![i, j];
            } else if arr_2d[i][j] == 'v' {
                dir = "down";
                pos = vec![i, j];
            } else if arr_2d[i][j] == '<' {
                dir = "left";
                pos = vec![i, j];
            } else if arr_2d[i][j] == '>' {
                dir = "right";
                pos = vec![i, j];
            }
        }
    }
    let starting_pos = pos.clone();

    
    let path = traverse(arr_2d.clone(), pos, dir);

    // Traverse the path
    fn traverse(arr_2d: Vec<Vec<char>>, mut pos: Vec<usize>, mut dir: &str,) -> Vec<Vec<usize>> {
        let starting_pos = pos.clone();
        let starting_dir = dir;
        let mut looping = true;
        let mut distinct_arr: Vec<Vec<usize>> = Vec::new();
        let mut direction_arr: Vec<&str> = Vec::new();
        let mut caught_in_loop = false;
        let mut overlapped = 0;
        while looping && !caught_in_loop {
            if !distinct_arr.contains(&vec![pos[0], pos[1]]) {
                distinct_arr.push(vec![pos[0], pos[1]]);
                overlapped = 0;
                direction_arr.push(dir);
            } else {
                overlapped += 1;
                if overlapped > distinct_arr.len() {
                    if pos == starting_pos && dir != starting_dir {
                        looping = false;
                    } else {
                        caught_in_loop = true;
                    }
                }
            }
            match dir {
                "up" => {
                    if pos[0] == 0 {
                        looping = false;
                    } else if arr_2d[pos[0] - 1][pos[1]] != '#' {
                        pos[0] -= 1;
                    } else if arr_2d[pos[0] - 1][pos[1]] == '#' {
                        dir = "right";
                    }
                }
                "right" => {
                    if pos[1] + 1 >= arr_2d[0].len() {
                        looping = false;
                    } else if pos[1] + 1 < arr_2d[0].len() && arr_2d[pos[0]][pos[1] + 1] != '#' {
                        pos[1] += 1;
                    } else if arr_2d[pos[0]][pos[1] + 1] == '#' {
                        dir = "down";
                    }
                }
                "down" => {
                    if pos[0] + 1 >= arr_2d.len() {
                        looping = false;
                    } else if arr_2d[pos[0] + 1][pos[1]] != '#' {
                        pos[0] += 1;
                    } else if arr_2d[pos[0] + 1][pos[1]] == '#' {
                        dir = "left";
                    }
                }
                "left" => {
                    if pos[1] == 0 {
                        looping = false;
                    } else if arr_2d[pos[0]][pos[1] - 1] != '#' {
                        pos[1] -= 1;
                    } else if arr_2d[pos[0]][pos[1] - 1] == '#' {
                        dir = "up";
                    }
                }
                &_ => {}
            }
        }
        if caught_in_loop {
            return vec![vec![0, 0]];
        }
        return distinct_arr;
    }
    println!("Number of distinct positions: {}", path.len());

    // Use the distinct positions, add '#' along each part of the path, and check if it creates a loop
    // For each pos in path, check if replacing that position with a '#' creates a loop
    let mut caught = 0;
    let mut i = 0;
    dir = "up";
    for path_pos in path.clone() {
        let mut arr_cpy = arr_2d.clone();
        arr_cpy[path_pos[0]][path_pos[1]] = '#';
        let res = traverse(arr_cpy, starting_pos.clone(), dir);
        if res[0] == vec![0, 0] {
            caught += 1;
            println!("Caught at position: {}, {}", path_pos[0], path_pos[1]);
        }
        i += 1;
        // println!("{}: {}", i, path.len());
    }
    println!("Number of positions that create a loop: {}", caught);
}
