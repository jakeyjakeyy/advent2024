use std::fs;
use regex::Regex;
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut active = true;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(&input) {
        if &cap[0] == "don't()" {
            active = false;
            continue;
        } else if &cap[0] == "do()" {
            active = true;
            continue;
        } else if active {
            sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        }
    }
    println!("Sum: {}", sum);
}
