use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = input.lines().collect();
    // 2D array is a vector ([]) or vectors ([][]) of characters ([0][0] = 'A')
    let mut array_2d: Vec<Vec<char>> = Vec::new();

    // Go through each line, convert it to a vector of characters, and add it to the 2D array
    for line in lines {
        array_2d.push(line.chars().collect());
    }

    let mut total = 0;
    for i in 1..array_2d.len() {
        for j in 1..array_2d[i].len() {
            let mut found = 0;
            // If we find an 'A', start searching
            if array_2d[i][j] == 'A' {
                // Make sure we don't break out of array bounds
                if i + 1 < array_2d.len() && j + 1 < array_2d[i].len() {
                    // Check MAS down and to the right (SAM up and to the left)
                    if array_2d[i-1][j-1] == 'M' && array_2d[i+1][j+1] == 'S' {
                        found += 1;
                    }
                    // Check SAM down and to the right (MAS up and to the left)
                    if array_2d[i-1][j-1] == 'S' && array_2d[i+1][j+1] == 'M' {
                        found += 1;
                    }
                    // Check MAS up and to the right (SAM down and to the left)
                    if array_2d[i+1][j-1] == 'M' && array_2d[i-1][j+1] == 'S' {
                        found += 1;
                    }
                    // Check SAM up and to the right (MAS down and to the left)
                    if array_2d[i+1][j-1] == 'S' && array_2d[i-1][j+1] == 'M' {
                        found += 1;
                    }
                    if found >=2 {
                        total += 1;
                    }
                }
            }
        }
    }
    println!("Total: {}", total);
}