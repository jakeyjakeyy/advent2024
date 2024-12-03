use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<&str> = line.split(" ").collect();
        let mut is_safe = true;
        let mut last = 0;
        let mut i = 0;
        for level in &levels {
            if i + 1 >= levels.len() {
                break;
            }
            let next_level = levels[i + 1];
            let mut diff = level.parse::<i32>().unwrap() - next_level.parse::<i32>().unwrap();
            if last < 0 && diff > 0 || last > 0 && diff < 0 {
                is_safe = false;
            }
            last = diff;
            diff = diff.abs();
            if diff > 3 || diff < 1 {
                is_safe = false;
            }
            i += 1;
        }
        if is_safe {
            safe += 1;
        }
    }
    println!("Safe: {}", safe);
}
