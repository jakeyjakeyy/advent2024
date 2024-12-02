use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let nums: Vec<&str> = line.rsplit("   ").collect();
        list1.push(nums[1].parse::<i32>().unwrap());
        list2.push(nums[0].parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut sum: i32 = 0;
    for i in 0..list1.len() {
        if list1[i] < list2[i] {
            sum += list2[i] - list1[i];
        } else {
            sum += list1[i] - list2[i];
        }
    }
    println!("The sum of the differences between the two lists is: {}", sum);
}

