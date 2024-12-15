use std::fs;
use regex::Regex;
fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let re = Regex::new(r"[XY][+=](\d+)").unwrap();
    let machines = input.split("\n\n").collect::<Vec<&str>>();

    let mut total_a = 0;
    let mut total_b = 0;
    for machine in machines {
        let mut regex_results = vec![];
        for cap in re.captures_iter(machine) {
            regex_results.push(cap[1].parse::<i32>().unwrap());
        }

        let ax: i64 = regex_results[0] as i64; // button a x
        let ay: i64 = regex_results[1] as i64; // button a y
        let bx: i64 = regex_results[2] as i64; // button b x
        let by: i64 = regex_results[3] as i64; // button b y
        let px: i64 = regex_results[4] as i64 + 10000000000000; // prize x
        let py: i64 = regex_results[5] as i64 + 10000000000000; // prize y


        let a = (px * by - py * bx) / (ax * by - ay * bx);
        let b = (ax * py - ay * px) / (ax * by - ay * bx);
        
        if (a * ax) + (b * bx) == px && (a * ay) + (b * by) == py {
            total_a += a * 3;
            total_b += b;
        }
    }
    let total = total_a + total_b;
    println!("Total: {}", total);
}