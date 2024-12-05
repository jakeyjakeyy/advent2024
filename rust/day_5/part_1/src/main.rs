use std::fs;
use regex::Regex;

fn main() {
    let input: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let separate: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Vec<&str>> = separate[0]
    .split("\n")
    .map(|s| s.split('|').collect())
    .collect();
    let updates: Vec<&str> = separate[1].split("\n").collect();


    let mut sum = 0;
    let mut counter = 0;
    for update in &updates {
        let mut fail = false;
        for rule in &rules {
            if fail {
                break;
            }
            let pattern = format!("{}|{}", rule[0], rule[1]);
            let re = Regex::new(&pattern).unwrap();
            
            let mut rule2_found = false;
            for cap in re.captures_iter(&update) {
                if &cap[0] == rule[0] {
                    if rule2_found {
                        fail = true;
                        break;
                    }
                } else if &cap[0] == rule[1] {
                    rule2_found = true;
                }
            }
        }
        if !fail {
            let vals = update.split(",").collect::<Vec<&str>>();
            let mid = vals.len() / 2;
            let val = vals[mid].parse::<i32>().unwrap();
            sum += val;
        }
        counter += 1;
        println!("{} of {}", counter, updates.len()); 
    }
    println!("{}", sum);

}
