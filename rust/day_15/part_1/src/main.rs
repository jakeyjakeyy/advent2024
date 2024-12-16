use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let split = input.split("\n\n").collect::<Vec<&str>>();

    let raw_map = split[0];
    let directions = split[1];

    let mut map: Vec<Vec<char>> = raw_map.lines().map(|x| x.chars().collect()).collect();
    let mut pos: (usize, usize) = (0, 0);

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                pos = (i, j);
            }
        }
    }

    for dir in directions.chars() {
        pos = handle_move(&mut map, pos, dir);
    }

    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                total += 100 * i + j;
            }
        }
    }
    println!("{}", total);
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
    if map[next_pos.0][next_pos.1] == 'O' {
        let res = handle_move(map, next_pos, dir);
        if res == next_pos {
            return pos;
        }
        handle_move(map, pos, dir);
    } else {
        let current_char = map[pos.0][pos.1];
        map[next_pos.0][next_pos.1] = current_char;
        map[pos.0][pos.1] = '.';
    }
    next_pos
}
