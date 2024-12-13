use std::fs;
use std::collections::HashMap;
fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut arr_2d: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        arr_2d.push(row);
    }
    let result = find_areas(arr_2d);
    println!("{:?}", result);
}

fn find_areas(arr_2d: Vec<Vec<char>>) -> i32 {
    let mut calculated_plots: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut total = 0;
    for y in 0..arr_2d.len() {
        for x in 0..arr_2d[y].len() {
            if calculated_plots.values().any(|v| v.contains(&(x, y))) {
                continue;
            }
            let current_char = arr_2d[y][x];
            let mut used_coords: Vec<(usize, usize)> = Vec::new();
            let res = calculate_plot(&arr_2d, (x, y), &mut used_coords);

            let mut geometric_sides = 0;
            for coord in &used_coords {
                calculated_plots.entry(current_char).or_insert(Vec::new()).push(*coord);
                geometric_sides += count_corners(&arr_2d, *coord);
            }
            total += geometric_sides * res.0;
        }
    }
    total
}

fn calculate_plot(arr_2d: &Vec<Vec<char>>, coord: (usize, usize), used_coords: &mut Vec<(usize, usize)>) -> (i32, i32) {
    let x = coord.0;
    let y = coord.1;
    let mut sides = 4;

    let mut plot_size = 1;

    let current_char = arr_2d[y][x];

    if used_coords.contains(&coord) {
        return (0, 0);
    }
    used_coords.push(coord);
    if y > 0 {
        let up = arr_2d[y - 1][x];
        if up == current_char {
            sides -= 1;
            let res = calculate_plot(arr_2d, (x, y - 1), used_coords);
            plot_size += res.0;
            sides += res.1;
        }
    }
    if y < arr_2d.len() - 1 {
        let down = arr_2d[y + 1][x];
        if down == current_char {
            sides -= 1;
            let res = calculate_plot(arr_2d, (x, y + 1), used_coords);
            plot_size += res.0;
            sides += res.1;
        }
    }
    if x > 0 {
        let left = arr_2d[y][x - 1];
        if left == current_char {
            sides -= 1;
            let res = calculate_plot(arr_2d, (x - 1, y), used_coords);
            plot_size += res.0;
            sides += res.1;
        }
    }
    if x < arr_2d[x].len() - 1 {
        let right = arr_2d[y][x + 1];
        if right == current_char {
            sides -= 1;
            let res = calculate_plot(arr_2d, (x + 1, y), used_coords);
            plot_size += res.0;
            sides += res.1;
        }
    }
    (plot_size, sides)
}

fn count_corners(arr_2d: &Vec<Vec<char>>, coord: (usize, usize)) -> i32 {
    let x = coord.0;
    let y = coord.1;
    let mut sides = 0;

    let current_char = arr_2d[y][x];

    let mut d1: char;
    let mut d2: char;
    let mut d3: char;

    if x == 0 || x == arr_2d[y].len() - 1 {
        if y < arr_2d.len() - 1 {
            if arr_2d[y + 1][x] != current_char {
                sides += 1;
            }
        } 
        if y > 0 {
            if arr_2d[y - 1][x] != current_char {
                sides += 1;
            }
        }
        if y == 0 || y == arr_2d.len() - 1 {
            sides += 1;
        }
    }
    if y == 0 || y == arr_2d.len() - 1 {
        if x == 0 {
            if arr_2d[y][x + 1] != current_char {
                sides += 1;
            }
        } else if x == arr_2d[y].len() - 1 {
            if arr_2d[y][x - 1] != current_char {
                sides += 1;
            }
        } else {
            if arr_2d[y][x - 1] != current_char {
                sides += 1;
            }
            if arr_2d[y][x + 1] != current_char {
                sides += 1;
            }
        }
    }

    // up and left
    if y > 0 && x > 0 {
        d1 = arr_2d[y - 1][x];
        d2 = arr_2d[y][x - 1];
        d3 = arr_2d[y - 1][x - 1];
        add_side(&mut sides, current_char, d1, d2, d3);
    }
    // up and right
    if y > 0 && x + 1< arr_2d[y].len() {
        d1 = arr_2d[y - 1][x];
        d2 = arr_2d[y][x + 1];
        d3 = arr_2d[y - 1][x + 1];
        add_side(&mut sides, current_char, d1, d2, d3);
    }
    // down and left
    if y + 1 < arr_2d.len() && x > 0 {
        d1 = arr_2d[y + 1][x];
        d2 = arr_2d[y][x - 1];
        d3 = arr_2d[y + 1][x - 1];
        add_side(&mut sides, current_char, d1, d2, d3);
    }
    // down and right
    if y + 1 < arr_2d.len() && x + 1 < arr_2d[y].len() {
        d1 = arr_2d[y + 1][x];
        d2 = arr_2d[y][x + 1];
        d3 = arr_2d[y + 1][x + 1];
        add_side(&mut sides, current_char, d1, d2, d3);
    }

    fn add_side(sides: &mut i32, current_char: char, d1: char, d2: char, d3: char) {
        if d1 != current_char && d2 != current_char {
            *sides += 1;
        } else if (d1 == current_char && d2 == current_char) && d3 != current_char {
            *sides += 1;
        }
    }
    sides
}