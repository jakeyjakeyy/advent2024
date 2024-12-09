use std::fs;
use std::collections::{HashMap, HashSet};
fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let lines = input.lines();
    let mut arr_2d: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let arr: Vec<char> = line.chars().collect();
        arr_2d.push(arr);
    }

    let mut coord_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (i, row) in arr_2d.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                coord_map.entry(c).or_insert(Vec::new()).push((i as i32, j as i32));
            }
        }
    }

    let mut antinodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut seen_tuples: HashSet<(i32, i32)> = HashSet::new();
    for (ch, coords) in coord_map.iter() {
        for (x,y) in coords {
            for (x2, y2) in coords.iter() {
                if x == x2 && y == y2 {
                    continue;
                }
                let mut i = 1;
                loop {
                    let mut changed = false;
                    let dx = (x - x2) * i;
                    let dx2 = (x2 - x) * i;
                    let dy = (y - y2) * i;
                    let dy2 = (y2 - y) * i;

                    let antinode_1 = (x + dx, y + dy);
                    let antinode_2 = (x2 + dx2, y2 + dy2);
                    
                    if i == 1 &&
                        !seen_tuples.contains(&(*x, *y)) {
                        antinodes.entry(*ch).or_insert(Vec::new()).push((*x, *y));
                        seen_tuples.insert((*x, *y));
                    }

                    if in_bounds(antinode_1.0, antinode_1.1, &arr_2d) &&
                        !seen_tuples.contains(&antinode_1) {
                            antinodes.entry(*ch).or_insert(Vec::new()).push(antinode_1);
                            seen_tuples.insert(antinode_1);
                            changed = true;
                        }
                    if in_bounds(antinode_2.0, antinode_2.1, &arr_2d) &&
                        !seen_tuples.contains(&antinode_2) {
                            antinodes.entry(*ch).or_insert(Vec::new()).push(antinode_2);
                            seen_tuples.insert(antinode_2);
                            changed = true;
                        }
                    i += 1; 
                    if !changed && 
                        !in_bounds(antinode_1.0, antinode_2.1, &arr_2d) &&
                        !in_bounds(antinode_2.0, antinode_2.1, &arr_2d) {
                        break;
                    }
                }
            }
        }
    }

    let mut total = 0;
    for (ch, coords) in antinodes {
        total += coords.len();
    }
    println!("{}", total);
}

fn in_bounds(x: i32, y: i32, arr: &Vec<Vec<char>>) -> bool {
    x >= 0 && x < arr[0].len() as i32 && y >= 0 && y < arr.len() as i32
}
