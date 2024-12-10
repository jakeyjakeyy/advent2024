use std::fs;
use std::collections::HashMap;
fn main() {
    let input = fs::read_to_string("src/input.sample.txt").unwrap();
    
    let mut files: Vec<i32> = Vec::new();
    let mut empty_spaces: Vec<i32> = Vec::new();
    let mut i = 0;
    for c in input.chars() {
        if i % 2 == 0 {
            files.push(c.to_digit(10).unwrap() as i32);
        } else {
            empty_spaces.push(c.to_digit(10).unwrap() as i32);
        }
        i += 1;
    }
    // println!("Files: {:?}", files);
    // println!("Empty spaces: {:?}", empty_spaces);

    let mut ordered_files: Vec<i32> = files.clone();
    let mut ordered_ids: Vec<i32> = (0..ordered_files.len() as i32).collect();
    files.reverse();

    let mut id = files.len() - 1;
    let mut offset = 0;
    for file in files {
        if id == 0 {
            break;
        }
        for i in 0..empty_spaces.len() {
            if empty_spaces[i] >= file &&
            i <= id {
                // println!("Empty spaces: {:?}", empty_spaces);
                println!("File: {} of size {} fitting into slot {} of size {}", id, file, i, empty_spaces[i]);
                let original_space = empty_spaces[i];
                if id >= empty_spaces.len() {
                    empty_spaces[id - 1] = file;
                    let original_file = ordered_files.pop();
                    ordered_files.insert(i , original_file.unwrap());
                } else {
                    // println!("removing empty space at index {}", id + 1 + offset);
                    // println!("turning empty space at index {} into {}", id + offset, empty_spaces[id + offset] + empty_spaces[id + 1 + offset] + file);
                    empty_spaces[id + offset] = empty_spaces[id + offset] + empty_spaces[id +  1 + offset] + file;
                    empty_spaces.remove(id + offset + 1);
                    offset = 1;
                    let original_file = ordered_files[id];
                    ordered_files.remove(id);
                    ordered_files.insert(i, original_file);
                }
                empty_spaces[i] = 0;
                empty_spaces.insert(i + 1, original_space - file);
                println!("Ordered files: {:?}", ordered_files);
                break;
            }
        }
        id -= 1;
    }

    println!("Ordered files: {:?}", ordered_files);
    println!("Empty spaces: {:?}", empty_spaces);

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
    println!("Input: {:?}", input);
    let mut reordered_input: Vec<String> = Vec::new();

    let mut replacement_index = input.len() - 1;
    while replacement_index > 0 {
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
            let freesize = calculate_size(input.clone(), i, false);
            println!("Freesize: {}", freesize);
            let filesize = calculate_size(input.clone(), replacement_index, true);
            println!("Filesize: {}", filesize);
            if freesize < filesize {
                reordered_input.push(input[i].clone());
                continue;
            }
            reordered_input.push(input[replacement_index].clone());
            replacement_index -= filesize as usize;
        }
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

fn calculate_size(input: Vec<String>, replacement_index: usize, get_filesize: bool) -> u64 {

    let mut index = replacement_index;
    let mut filesize: u64 = 0;
    let file_content = input[index].clone();
    loop {
        if input[index] == file_content {
            filesize += 1;
        } else {
            break;
        }
        if get_filesize {
            index -= 1;
        } else {
            index += 1;
        }
    }
    return filesize;
}