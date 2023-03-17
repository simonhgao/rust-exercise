use std::io::{self, BufRead};

fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for num in nums {
        match sum.checked_add(*num) {
            Some(val) => sum = val,
            None => return None,
        }
    }
    Some(sum)
}

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    println!("Please input your u32 list divided by space");
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate sum of the numbers
    match sum_u32(&nums) {
        Some(sum) => println!("Sum of the numbers is {}", sum),
        None => println!("Overflow occurred"),
    }
}
