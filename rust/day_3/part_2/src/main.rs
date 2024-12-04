use std::fs;
use regex::Regex;
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    // First lets find 'dont()' and remove everything between it and the next 'do()'
    let re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();

    let input = re.replace_all(&input, "");
    println!("{}", input);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(&input) {
        sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
    }
    println!("Sum: {}", sum);
}
