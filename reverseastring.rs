//Reverse a String

use std::io;

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    println!("Reversed string: {}", reverse_string(input));
}
