use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = input.lines().collect();
    let mut array_2d: Vec<Vec<char>> = Vec::new();

    for line in lines {
        array_2d.push(line.chars().collect());
    }

    let search_term = "XMAS".chars().collect::<Vec<char>>();

    let mut total = 0;
    for i in 0..array_2d.len() {
        for j in 0..array_2d[i].len() {
            if array_2d[i][j] == search_term[0] {
                let found = search_array(&array_2d, &search_term, i, j);
                if found > 0 {
                    println!("Found at: {}, {}", i, j);
                }
                total += found;
            }
        }
    }
    println!("Total: {}", total);
}

fn search_array(array_2d: &Vec<Vec<char>>, search_term: &Vec<char>, i: usize, j: usize) -> usize {
    let mut found = 0;
    let term_len = search_term.len();

    // check left
    if j >= term_len - 1 {
        let mut found_left = true;
        for k in 0..term_len {
            if array_2d[i][j - k] != search_term[k] {
                found_left = false;
                break;
            }
        }
        if found_left {
            found += 1;
        }
    }

    // check right
    if j + term_len <= array_2d[i].len() {
        let mut found_right = true;
        for k in 0..term_len {
            if array_2d[i][j + k] != search_term[k] {
                found_right = false;
                break;
            }
        }
        if found_right {
            found += 1;
        }
    }

    // check up
    if i >= term_len - 1 {
        let mut found_up = true;
        for k in 0..term_len {
            if array_2d[i - k][j] != search_term[k] {
                found_up = false;
                break;
            }
        }
        if found_up {
            found += 1;
        }
    }

    // check down
    if i + term_len <= array_2d.len() {
        let mut found_down = true;
        for k in 0..term_len {
            if array_2d[i + k][j] != search_term[k] {
                found_down = false;
                break;
            }
        }
        if found_down {
            found += 1;
        }
    }

    // check diagonal up left
    if i >= term_len - 1 && j >= term_len - 1 {
        let mut found_diagonal_up_left = true;
        for k in 0..term_len {
            if array_2d[i - k][j - k] != search_term[k] {
                found_diagonal_up_left = false;
                break;
            }
        }
        if found_diagonal_up_left {
            found += 1;
        }
    }

    // check diagonal up right
    if i >= term_len - 1 && j + term_len <= array_2d[i].len() {
        let mut found_diagonal_up_right = true;
        for k in 0..term_len {
            if array_2d[i - k][j + k] != search_term[k] {
                found_diagonal_up_right = false;
                break;
            }
        }
        if found_diagonal_up_right {
            found += 1;
        }
    }

    // check diagonal down left
    if i + term_len <= array_2d.len() && j >= term_len - 1 {
        let mut found_diagonal_down_left = true;
        for k in 0..term_len {
            if array_2d[i + k][j - k] != search_term[k] {
                found_diagonal_down_left = false;
                break;
            }
        }
        if found_diagonal_down_left {
            found += 1;
        }
    }

    // check diagonal down right
    if i + term_len <= array_2d.len() && j + term_len <= array_2d[i].len() {
        let mut found_diagonal_down_right = true;
        for k in 0..term_len {
            if array_2d[i + k][j + k] != search_term[k] {
                found_diagonal_down_right = false;
                break;
            }
        }
        if found_diagonal_down_right {
            found += 1;
        }
    }

    found
}