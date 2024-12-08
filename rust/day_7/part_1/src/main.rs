use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for line in lines {
        let equation: Vec<&str> = line.split(":").collect();
        let total = equation[0].parse::<u64>().unwrap();
        let arr: Vec<u64> = equation[1].trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

        for i in 0..2_u32.pow((arr.len() - 1) as u32) {
            let mut calculated_total = arr[0];
            let mut bitmask = i;
            for n in &arr[1..] {
                if bitmask & 1 == 0 {
                    // println!("{} + {}", calculated_total, n);
                    calculated_total += n;
                } else {
                    // println!("{} * {}", calculated_total, n);
                    calculated_total *= n
                }
                bitmask >>= 1;
                // println!("calc: {}", calculated_total);
                if calculated_total > total {
                    break;
                }
            }
                if calculated_total == total {
                    result += total;
                    break;
                }
        }
    }
    println!("Result: {}", result);
}
