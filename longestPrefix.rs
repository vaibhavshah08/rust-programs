//Implement a function that finds the longest common prefix of a given set of strings.

use std::io;

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = String::new();
    for (i, c) in strings[0].chars().enumerate() {
        if strings[1..].iter().all(|s| s.chars().nth(i) == Some(c)) {
            prefix.push(c);
        } else {
            break;
        }
    }
    prefix
}

fn main() {
    println!("Enter a set of strings (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let strings: Vec<&str> = input.trim().split_whitespace().collect();
    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}
