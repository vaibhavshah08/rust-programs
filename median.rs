//Given a sorted array of integers, implement a function that returns the median of the array.

use std::io;

fn median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (nums[mid - 1] + nums[mid]) as f64 / 2.0
    } else {
        nums[len / 2] as f64
    }
}

fn main() {
    println!("Enter a sorted array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("Median: {}", median(&nums));
}
