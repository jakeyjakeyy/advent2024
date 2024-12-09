use std::fs;
use std::collections::HashMap;
fn main() {
    let input = fs::read_to_string("src/input.sample.txt").unwrap();
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

    let mut antinodes: Vec<(char, Vec<(i32, i32)>)> = Vec::new();
    for (ch, coords) in coord_map.iter() {
        for (x,y) in coords {
            for (x2, y2) in coords.iter() {
                if x == x2 && y == y2 {
                    continue;
                }
                let dx = x - x2;
                let dx2 = x2 - x;
                let dy = y - y2;
                let dy2 = y2 - y;

                let antinode_1 = (x + dx, y + dy);
                let antinode_2 = (x2 + dx2, y2 + dy2);

                if antinode_1.0 >= 0 && antinode_1.0 < arr_2d[0].len() as i32 &&
                    antinode_1.1 >= 0 && antinode_1.1 < arr_2d.len() as i32 {
                    if !antinodes.iter().any(|(ch, coords)| coords.contains(&antinode_1)) {
                        antinodes.push((*ch, vec![antinode_1]));
                    }
                }
                if antinode_2.0 >= 0 && antinode_2.0 < arr_2d[0].len() as i32 &&
                    antinode_2.1 >= 0 && antinode_2.1 < arr_2d.len() as i32 {
                    if !antinodes.iter().any(|(ch, coords)| coords.contains(&antinode_2)) {
                        antinodes.push((*ch, vec![antinode_2]));
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
