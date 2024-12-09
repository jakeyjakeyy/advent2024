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
    let mut reordered_input: Vec<String> = Vec::new();

    let mut replacement_index = input.len() - 1;
    for i in 0..input.len() {
        if replacement_index < i {
            break;
        }
        if input[i] != "." {
            reordered_input.push(input[i].clone());
            continue;
        }
        while input[replacement_index] == "." && replacement_index > i {
            replacement_index -= 1;
        }
        reordered_input.push(input[replacement_index].clone());
        replacement_index -= 1;
    }
    reordered_input
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