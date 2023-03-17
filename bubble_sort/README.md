# Rust Bubble Sort

This is a Rust implementation of the bubble sort algorithm, which is a simple sorting algorithm that repeatedly steps through a list, compares adjacent elements and swaps them if they are in the wrong order.

## Requirements

To run this program, you'll need to have Rust installed on your system.

## Usage

To run the program, open a terminal window and navigate to the project directory. Then, run the following command:

bash
Copy code
cargo run -- -basic (or --advanced)
The program will prompt you to enter a list of integers or string and other types who implement 'PartialOrd', separated by spaces. Once you've entered the list, the program will sort it using the bubble sort algorithm and print the sorted list to the console.

## Example

Here's two example of how to use the program:

-basic
```
$ cargo run -- -basic   
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/bubble_sort -basic`
2 -3 -4 2 100
Sorted input: [-4, -3, 2, 2, 100]
```
-advanced
```
$ cargo run -- -advanced
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/bubble_sort -advanced`
1.0 2.0 5.0 -1.0 0.2
Sorted input: ["-1.0", "0.2", "1.0", "2.0", "5.0"]
```
