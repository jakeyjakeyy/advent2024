use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut total_stones: u64 = input.split(" ").count() as u64;
    let amt = input.split(" ").count();
    let mut i = 0;
    for stone in input.split(" ") {
        let value = stone.parse::<u64>().unwrap();
        let blinks = 75;
        
        total_stones += blink(value, blinks);
        i += 1;
        println!("{}/{}: {}", i, amt, total_stones);
    }
    println!("Total stones: {}", total_stones);
}

fn blink(value: u64, blinks: i32) -> u64 {
    if blinks == 0 {
        return 0;
    }
    let mut new_stones: u64 = 0;
    if value == 0 {
        new_stones += blink(1, blinks - 1);
    } else if value.to_string().len() % 2 == 0 {
        let value_str = value.to_string();
        new_stones += 1;
        let stone_left = value_str[0..value_str.len() / 2].parse::<u64>().unwrap();
        let stone_right = value_str[value_str.len() / 2..].parse::<u64>().unwrap();
        new_stones += blink(stone_left, blinks - 1);
        new_stones += blink(stone_right, blinks - 1);
    } else {
        new_stones += blink(value * 2024, blinks - 1);
    }
    new_stones
}