use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<&str> = line.split_whitespace().collect();
        if is_safe(&levels, false) {
            safe += 1;
        }
    }
    println!("Safe: {}", safe);
}

fn is_safe(levels: &[&str], recursion: bool) -> bool {
    fn try_remove_and_check(levels: &[&str], index: usize) -> bool {
        let mut new_levels = levels.to_vec();
        new_levels.remove(index);
        is_safe(&new_levels, true)
    }

    let mut last_diff = 0;

    for i in 0..levels.len() - 1 {
        let current = levels[i].parse::<i32>().unwrap();
        let next = levels[i + 1].parse::<i32>().unwrap();
        let diff = current - next;

        // Check for ascending/descending violations
        if (last_diff < 0 && diff > 0) || (last_diff > 0 && diff < 0) {
            if recursion {
                return false;
            }
            return try_remove_and_check(levels, i.saturating_sub(1))
                || try_remove_and_check(levels, i)
                || try_remove_and_check(levels, i + 1);
        }

        // Check for range violations
        if diff.abs() > 3 || diff.abs() < 1 {
            if recursion {
                return false;
            }
            return try_remove_and_check(levels, i) || try_remove_and_check(levels, i + 1);
        }

        last_diff = diff;
    }

    true
}
