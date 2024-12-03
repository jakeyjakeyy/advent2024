use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<&str> = line.split(" ").collect();
        if is_safe(levels.clone(), false) {
            println!("Safe: {levels:?}");
            safe += 1;
        } else {
            println!("Unsafe: {levels:?}");
        }
    }
    println!("Safe: {}", safe);

    fn is_safe(levels: Vec<&str>, recursion: bool) -> bool {
        println!("Levels: {levels:?}");
        let mut last = 0;
        let mut i = 0;
        for level in &levels {
            println!("i: {i}");
            if i + 1 >= levels.len() {
                break;
            }
            let next_level = levels[i + 1];
            let mut diff = level.parse::<i32>().unwrap() - next_level.parse::<i32>().unwrap();
            if last < 0 && diff > 0 || last > 0 && diff < 0 {
                if recursion {
                    return false;
                }
                let mut new_levels = levels.clone();
                new_levels.remove(i - 1);
                if !is_safe(new_levels, true) {
                    new_levels = levels.clone();
                    new_levels.remove(i);
                    if !is_safe(new_levels, true) {
                        new_levels = levels.clone();
                        new_levels.remove(i + 1);
                        return is_safe(new_levels, true);
                    }
                } else {
                    return true;
                }
            }
            last = diff;
            diff = diff.abs();
            if diff > 3 || diff < 1 {
                if recursion {
                    return false;
                }
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if !is_safe(new_levels, true) {
                    new_levels = levels.clone();
                    new_levels.remove(i + 1);
                    return is_safe(new_levels, true);
                } else {
                    return true;
                }
            }
            i += 1;
        }
        true
    }
}
