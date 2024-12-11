use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut total_stones: u64 = input.split(" ").count() as u64;
    let amt = input.split(" ").count();
    let mut i = 0;
    for stone in input.split(" ") {
        let value = stone.parse::<u64>().unwrap();
        let blinks = 75;
        
        total_stones += blink_controller(value, blinks);
        i += 1;
        println!("{}/{}: {}", i, amt, total_stones);
    }
    println!("Total stones: {}", total_stones);
}

fn blink(value: u64, blinks: i32, cache: &mut HashMap<(u64, i32), u64>) -> u64 {
    if blinks == 0 {
        return 0;
    }
    // Check cache
    if let Some(&result) = cache.get(&(value, blinks)) {
        return result;
    }
    let mut new_stones: u64 = 0;
    if value == 0 {
        new_stones += blink(1, blinks - 1, cache);
    } else {
        let digit_count = if value == 0 { 1 } else { (value as f64).log10().floor() as u32 + 1 };
        if digit_count % 2 == 0 {
            let divisor = 10u64.pow(digit_count / 2);
            let stone_left = value / divisor;
            let stone_right = value % divisor;
            new_stones += 1;
            new_stones += blink(stone_left, blinks - 1, cache);
            new_stones += blink(stone_right, blinks - 1, cache);
        } else {
            new_stones += blink(value * 2024, blinks - 1, cache);
        }
    }
    cache.insert((value, blinks), new_stones);
    new_stones
}

fn blink_controller(value: u64, blinks: i32) -> u64 {
    let mut cache = HashMap::new();
    blink(value, blinks, &mut cache)
}