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

    // Traverse the path
    let mut looping = true;
    let mut distinct_arr: Vec<Vec<usize>> = Vec::new();
    while looping {
        if !distinct_arr.contains(&vec![pos[0], pos[1]]) {
            distinct_arr.push(vec![pos[0], pos[1]]);
        }
        match dir {
            "up" => {
                if pos[0] - 1 < 0 {
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
                if pos[1] - 1 < 0 {
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
    println!("Number of distinct positions: {}", distinct_arr.len());
}
