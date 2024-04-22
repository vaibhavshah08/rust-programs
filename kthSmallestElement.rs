//Implement a function that returns the kth smallest element in a given array.

use std::io;

fn kth_smallest(nums: &[i32], k: usize) -> Option<i32> {
    nums.iter().copied().nth(k - 1)
}

fn main() {
    println!("Enter an array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Please enter a valid number");

    if let Some(smallest) = kth_smallest(&nums, k) {
        println!("{}th smallest element: {}", k, smallest);
    } else {
        println!("Array too short or k too large");
    }
}
