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

        let ax: i32 = regex_results[0]; // button a x
        let ay: i32 = regex_results[1]; // button a y
        let bx: i32 = regex_results[2]; // button b x
        let by: i32 = regex_results[3]; // button b y
        let px: i32 = regex_results[4]; // prize x
        let py: i32 = regex_results[5]; // prize y


        let mut current_x = 0;
        let mut current_y = 0;

        for i in 0..100 {
            for j in 0..100 {
                current_x = (ax * i) + (bx * j);
                current_y = (ay * i) + (by * j);
                
                if current_x > px || current_y > py {
                    break;
                }
                if current_x == px && current_y == py {
                    total_a += i;
                    total_b += j;
                    break;
                }
            }
        }
    }
    total_a = total_a * 3;
    let total = total_a + total_b;
    println!("Total: {}", total);
}
