use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for line in lines {
        let equation: Vec<&str> = line.split(":").collect();
        let total = equation[0].parse::<u64>().unwrap();
        let arr: Vec<u64> = equation[1].trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

        for mut i in 0..3_u32.pow((arr.len() - 1) as u32) {
            let mut calculated_total = arr[0];
            for n in &arr[1..] {
                let operation = i % 3;
                i /= 3;
                match operation{
                    0 => calculated_total += n,
                    1 => calculated_total *= n,
                    2 => calculated_total = (calculated_total.to_string() + &n.to_string()).parse::<u64>().unwrap(),
                    _ => println!("Error"),
                }
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
