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
            for coord in used_coords {
                calculated_plots.entry(current_char).or_insert(Vec::new()).push(coord);
            }
            total += res.0 * res.1;
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