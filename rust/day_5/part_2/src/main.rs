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


    let mut failures: Vec<&str> = Vec::new();
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
                        failures.push(update);
                        break;
                    }
                } else if &cap[0] == rule[1] {
                    rule2_found = true;
                }
            }
        }
        // if !fail {
        //     let vals = update.split(",").collect::<Vec<&str>>();
        //     let mid = vals.len() / 2;
        //     let val = vals[mid].parse::<i32>().unwrap();
        //     sum += val;
        // }
        counter += 1;
        println!("{} of {}", counter, updates.len()); 
    }

    println!("Failures: {}", failures.len());
    let mut sum = 0;
    for loser in failures {
        let vals = loser.split(",").collect::<Vec<&str>>();
        let swapped = swapcheck(vals, &rules);
        let mid = swapped.len() / 2;
        let val = swapped[mid].parse::<i32>().unwrap();
        sum += val;
    }
    println!("Sum: {}", sum);

    fn swapcheck<'a>(vals: Vec<&'a str>, rules: &Vec<Vec<&'a str>>) -> Vec<&'a str> {
        let mut newvals = vals.clone();
        for rule in rules {
            let mut rule2_location = 0;
            let mut rule2_found = false;
            for i in 0..vals.len() {
                if vals[i] == rule[0] {
                    if rule2_found {
                        // swap
                        let temp = vals[i];
                        newvals[i] = vals[rule2_location];
                        newvals[rule2_location] = temp;
                        return swapcheck(newvals, rules);
                    }
                } else if vals[i] == rule[1] {
                    rule2_found = true;
                    rule2_location = i;
                }
            }
        }
        return newvals
    }
}