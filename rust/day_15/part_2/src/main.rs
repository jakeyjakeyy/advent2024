use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.sample.txt").unwrap();
    let split = input.split("\n\n").collect::<Vec<&str>>();

    let raw_map = split[0];
    let directions = split[1];

    let mut map: Vec<Vec<char>> = raw_map.lines().map(|x| x.chars().collect()).collect();
    let mut pos: (usize, usize) = (0, 0);

    println!("{:?}", map);
    println!();
    map = double_map(map);

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                pos = (i, j);
            }
        }
    }

    for line in &map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    for dir in directions.chars() {
        pos = handle_move(&mut map, pos, dir);
        for line in &map {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
        let mut uinput = String::new();
        std::io::stdin().read_line(&mut uinput).unwrap();
    }

    // let mut total = 0;
    // for i in 0..map.len() {
    //     for j in 0..map[i].len() {
    //         if map[i][j] == 'O' {
    //             total += 100 * i + j;
    //         }
    //     }
    // }
    // println!("{}", total);
}

fn double_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::with_capacity(map.len());
    for i in 0..map.len() {
        let mut new_row = Vec::with_capacity(map[i].len() * 2);
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else if map[i][j] == '@' {
                new_row.push(map[i][j]);
                new_row.push('.');
            } else {
                new_row.push(map[i][j]);
                new_row.push(map[i][j]);
            }
        }
        new_map.push(new_row);
    }
    new_map
}

fn handle_move(map: &mut Vec<Vec<char>>, pos: (usize, usize), dir: char) -> (usize, usize) {
    let mut next_pos = pos;
    match dir {
        '<' => next_pos.1 -= 1,
        '>' => next_pos.1 += 1,
        '^' => next_pos.0 -= 1,
        'v' => next_pos.0 += 1,
        _ => (),
    }
    if map[next_pos.0][next_pos.1] == '#' {
        return pos;
    }
    if map[next_pos.0][next_pos.1] == '[' || map[next_pos.0][next_pos.1] == ']' {
        if dir == '<' || dir == '>' {
            let res = handle_move(map, next_pos, dir);
            if res == next_pos {
                return pos;
            }
            handle_move(map, pos, dir);
        } else {
            let second_half: (usize, usize);
            match map[next_pos.0][next_pos.1] {
                '[' => second_half = (next_pos.0, next_pos.1 + 1),
                ']' => second_half = (next_pos.0, next_pos.1 - 1),
                _ => panic!("Invalid char"),
            }
            let res = handle_move(map, next_pos, dir);
            if res == next_pos {
                return pos;
            }
            let res = handle_move(map, second_half, dir);
            if res == second_half {
                return pos;
            }
            handle_move(map, pos, dir);
        }
    } else {
        let current_char = map[pos.0][pos.1];
        map[next_pos.0][next_pos.1] = current_char;
        map[pos.0][pos.1] = '.';
    }
    next_pos
}
