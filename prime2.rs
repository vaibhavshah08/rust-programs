// Check Prime number (Same as Que 4)

use std::io;

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: u64 = input.trim().parse().expect("Please enter a valid number");
    println!("Is {} prime? {}", num, is_prime(num));
}
