//Checks weather a given number is prime or not

use std::io;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
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
