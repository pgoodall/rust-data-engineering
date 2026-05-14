/*
This example code counts the frequency of each number in the vector.
 */
use clap::Parser;
use hashmap_count::{Config, frequency, validate_num_list};

// Might move this to lib.rs
fn strip_punctuation(s: &str) -> String {
    s.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
}

fn main() {
    let args = Config::parse();

    if let Some(w) = args.word {
        if let Some(s) = args.sentence {
            println!("Checking how often {} appears in your sentence!", w)
        } else if let Some(f) = &args.file {
            if let Ok(contents) = hashmap_count::load_file(f) {
                // Might move this to lib.rs
                let mut count = 0;
                let lines = contents.lines();
                for line in lines {
                    count += line.split_whitespace()
                        .filter(| item | strip_punctuation(item).to_lowercase() == w )
                        .count();
                }
                println!("The word \"{}\" occurs {} times in the file {:?}", w, count, &args.file);
            } else {
                eprint!("Error: Could not read file.");
            }
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
        let sentence: Vec<String> = s
                                    .split_whitespace()
                                    .map(|x| strip_punctuation(x))
                                    .collect();
        let map = frequency(sentence);
        let mut result: Vec<String> = Vec::new();
        for (key, frequency) in map {
            result.push(format!("{}: {}", key, frequency))
        }
        println!(
            "The frequncy of each word in the vector is:\n{:#?}",
            result);
    };
    
}
