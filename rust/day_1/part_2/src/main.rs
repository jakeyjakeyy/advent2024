use std::fs;
use std::collections::HashMap;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let nums: Vec<&str> = line.rsplit("   ").collect();
        list1.push(nums[1].parse::<i32>().unwrap());
        list2.push(nums[0].parse::<i32>().unwrap());
    }

    let mut hashmap = HashMap::new();
    // populate hashmap
    for i in 0..list2.len() {
        if hashmap.contains_key(&list2[i]) {
            hashmap.insert(list2[i], hashmap.get(&list2[i]).unwrap() + 1);
            continue;
        }
        hashmap.insert(list2[i], 1);
    }
    let mut sum: i32 = 0;
    for i in 0..list1.len() {
        // for each number in list 1
        let num1 = list1[i];
        // check matches in hashmap
        if hashmap.contains_key(&num1) {
            let count = hashmap.get(&num1).unwrap();
            sum += num1 * count;
        }
    }
    let duration = start.elapsed();
    println!("Similarity score: {}", sum);
    println!("Time elapsed: {:?}", duration);
}

