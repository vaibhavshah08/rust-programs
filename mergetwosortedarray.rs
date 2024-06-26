//Merge 2 sorted arrays

use std::io;

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);
    result
}

fn main() {
    println!("Enter the first sorted array of integers (space-separated):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let arr1: Vec<i32> = input1.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("Enter the second sorted array of integers (space-separated):");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let arr2: Vec<i32> = input2.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged sorted arrays: {:?}", merged);
}
