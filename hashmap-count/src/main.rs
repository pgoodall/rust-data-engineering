/*
This example code counts the frequency of each number in the vector.
 */
use clap::Parser;
use hashmap_count::{frequency, Config};

fn main() {
    let args = Config::parse();
    
    let numbers: Vec<i32> = args.num_list
                            .unwrap()
                            .split(",")
                            .map(| x | x.trim().parse::<i32>()
                            .unwrap())
                            .collect();
    //println!("{:?}", numbers);
    
    //let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let map = frequency(numbers);
    let mut result = Vec::new();
    for (key, frequency) in map {
        result.push(format!("{}: {}", key, frequency))
    }

    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is:\n{:?}",
        result);
}
