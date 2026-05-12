/*
This example code counts the frequency of each number in the vector.
 */
use clap::Parser;
use hashmap_count::{Config, frequency, validate_num_list};

fn main() {
    let args = Config::parse();

    if let Some(w) = args.word {
        if let Some(s) = args.sentence {
            println!("Checking how often {} appears in your sentence!", w)
        } else if let Some(f) = args.file {
            println!("Checking how often {} appears in your file!", w)
        } else {
            println!("Error: The 'word' option must be set with either a sentence or a file.");
            std::process::exit(1)
        };
    } else if let Some(n) = args.num_list {
        let numbers = validate_num_list(n);
        let map = frequency(numbers);
        let mut result = Vec::new();
        for (key, frequency) in map {
            result.push(format!("{}: {}", key, frequency))
        }
        println!(
            "The frequency of each number in the vector is:\n{:?}",
            result);
    } else if let Some(s) = args.sentence {
        let sentence: Vec<&str> = s.split_whitespace().map(|x| x.trim()).collect();
        let map = frequency(sentence);
        let mut result: Vec<String> = Vec::new();
        for (key, frequency) in map {
            result.push(format!("{}: {}", key, frequency))
        }
        println!(
            "The frequncy of each word in the vector is:\n{:?}",
            result);
    };

    
    // let map = frequency(numbers);
    // let mut result = Vec::new();
    // for (key, frequency) in map {
    //     result.push(format!("{}: {}", key, frequency))
    // }

    //print the results in a human readable format that explains what the result is.
    
}
