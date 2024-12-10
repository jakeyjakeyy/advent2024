use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let parsed_input: Vec<String>;

    parsed_input = parse_input(&input);

    let reordered_input = reorder_input(parsed_input);
    
    let checksum = calculate_checksum(reordered_input);
    println!("Checksum: {}", checksum);
}

fn parse_input(input: &str) -> Vec<String> {
    let mut parsed_input: Vec<String> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut files_found = 0;
    let mut i = 0;
    for c in chars {
        if i % 2 == 0 {
            let active_char = files_found.to_string();
            files_found += 1;
            for _ in 0..c.to_digit(10).unwrap() {
                parsed_input.push(active_char.clone());
            }
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                parsed_input.push(".".to_string());
            }
        }
        i += 1;
    }
    parsed_input
}

fn reorder_input(input: Vec<String>) -> Vec<String> {
    let mut reordered_input: Vec<String> = input.clone();

    let mut replacement_index = input.len() - 1;
    let mut id: u32 = input[replacement_index].parse::<u32>().unwrap();
    let mut i = 0;
    while i < input.len() {
        if replacement_index == 0 {
            break;
        }
        if i == reordered_input.len() - 1 &&
        replacement_index != 0 ||
        i >= replacement_index {
                replacement_index -= calculate_file_size(&reordered_input, replacement_index) as usize;
                i = 0;
                loop {
                    if input[replacement_index] == "." {
                        replacement_index -= 1;
                    } else {
                        break;
                    }
                }
                id = input[replacement_index].parse::<u32>().unwrap();
        }
        if reordered_input[i] != "." {
            i += 1;
            continue;
        }
 
        let free_space = calculate_free_space(&reordered_input, i);
        let file_size = calculate_file_size(&reordered_input, replacement_index);
        if free_space >= file_size {
            for j in 0..file_size {
                reordered_input[replacement_index - j as usize] = ".".to_string();
                reordered_input[i + j as usize] = id.to_string();
            }
            replacement_index -= file_size as usize;
            loop {
                if input[replacement_index] == "." {
                    replacement_index -= 1;
                } else {
                    break;
                }
            }

            id = input[replacement_index].parse::<u32>().unwrap();
            i = 0;
        } else {
            i += 1;
        }
    }
    reordered_input
}

fn calculate_free_space(input: &Vec<String>, index: usize) -> u32 {
    let mut free_space = 0;
    let mut i = index;
    while i < input.len() && input[i] == "." {
        free_space += 1;
        i += 1;
    }
    free_space
}

fn calculate_file_size(input: &Vec<String>, index: usize) -> u32 {
    let mut file_size = 0;
    let case = &input[index];
    let mut i = index;
    while i > 0 && input[i] == *case {
        file_size += 1;
        i -= 1;
    }
    file_size
}

fn calculate_checksum(input: Vec<String>) -> u64 {
    let mut checksum: u64 = 0;

    for i in 0..input.len() {
        if input[i] == "." {
            continue;
        }
        // checksum += ((char_to_i32(input[i])) * i as i32) as u64;
        checksum += input[i].parse::<u64>().unwrap() * i as u64;
    }
    checksum
}