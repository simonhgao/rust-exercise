use std::env;
mod sort; // import sort mod

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <flag>", args[0]);
        return;
    }

    let flag = &args[1];

    match flag.as_str() {
        "-basic" => {
            let mut input = read_input::<i32>();
            sort::basic_bubble_sort(&mut input);
            println!("Sorted input: {:?}", input);
        }
        "-advanced" => {
            let mut input = read_input::<String>();
            sort::advanced_bubble_sort(&mut input);
            println!("Sorted input: {:?}", input);
        }
        _ => println!("Unknown flag: {}", flag),
    }
}

fn read_input<T: std::str::FromStr>() -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}