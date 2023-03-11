use std::io;
mod sort; // import sort mod

fn main() {
    let mut input = String::new();
    println!("Please enter a list of integers separated by spaces:");
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut nums_copy = nums.clone();
    sort::bubble_sort(&mut nums_copy); 
    println!("Sorted array: {:?}", nums_copy);
}