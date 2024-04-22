//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number

use std::io;

fn first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    nums.iter().position(|&x| x == target)
}

fn main() {
    let mut nums = String::new();
    println!("Enter a sorted array of integers (space-separated):");
    io::stdin().read_line(&mut nums).expect("Failed to read line");
    let nums: Vec<i32> = nums.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("Enter the number to search:");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target: i32 = target.trim().parse().unwrap();

    if let Some(index) = first_occurrence(&nums, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
